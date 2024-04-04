import requests
import os
path = os.path.dirname(os.path.abspath(__file__))

VERIFY_CERT = path + "/../svp-backend/cert.pem"

# BACKEND TODO:
# - Creating a new pet and specifiying it's yard, either by given name or uuid, does not add that pet to that
#   yard

# ==============================Viewing Funcitons==================================
def view_pets(server, user_content, uuid , user_token):
    for pet_uuid in user_content["pets"]:
        response = requests.get(server + 'users/' + uuid + '/pets/' + pet_uuid, verify=VERIFY_CERT, headers={'X-Auth-Key': user_token})
        response_content = response.json()
        #This should display yard names, not uuid which is what pets will ultimately contain
        print("Pet: " + response_content['name'] + " Yard: " + response_content['pet_yard'])

def view_joined_yards(server, user_content, uuid , user_token):
    for pet_yard_uuid in user_content["joined_pet_yards"]:
        response = requests.get(server + 'users/' + uuid + '/pets_yards/' + pet_yard_uuid, verify=VERIFY_CERT, headers={'X-Auth-Key': user_token})
        response_content = response.json()
        print("Yard: " + response_content["name"])

def view_owned_yards(server, user_content, uuid , user_token):
    for yard_uuid in user_content['owned_pet_yards']:
        response = requests.get(server + 'users/' + uuid + '/pet_yards/' + yard_uuid, verify=VERIFY_CERT, headers={'X-Auth-Key': user_token})
        response_content = response.json()
        print("Yard: " + response_content["name"])

# =================================================================================


# ==============================Checking Funcitons==================================


def check_pet_name_in_yard(server, name, yard_uuid, uuid, user_token): 

    response = requests.get(server + 'users/' + uuid + '/pet_yards/' + yard_uuid, verify=VERIFY_CERT, headers={'X-Auth-Key': user_token})
    response_content = response.json()
    for pet_uuid in response_content['pets']:
        response = requests.get(server + 'users/' + uuid + '/pets/' + pet_uuid, verify=VERIFY_CERT, headers={'X-Auth-Key': user_token})
        response_content = response.json()
        if response_content['name'] == name:
            return response_content[uuid]

    return False

#Returns the uuid of an existing yard by name, or false if no such yard exists
def check_yard_name(server, name, user_content, uuid, user_token): 
    for yard_uuid in user_content['owned_pet_yards']:
        response = requests.get(server + 'users/' + uuid + '/pet_yards/' + yard_uuid, verify=VERIFY_CERT, headers={'X-Auth-Key': user_token})
        response_content = response.json()
        if name == response_content['name']:
            return response_content['uuid']
    return False

# =================================================================================


# ==============================Creation Funcitons==================================

def create_pet(server, user_content, uuid, user_token):
    yard_uuid = ""
    while True:
        #This will be the name of the yard initially, and if the yard exists it is set to the uuid
        yard_uuid = input("Pet yard: ");
        if check_yard_name(server, yard_uuid, user_content, uuid, user_token):
            yard_uuid = check_yard_name(server, yard_uuid, user_content, uuid, user_token)
            break
        else:
            print("There is no yard with this name")

    while True:
        name = input("Pet name: ");
        if check_pet_name_in_yard(server, name, yard_uuid, uuid, user_token):
            print("There is already a pet with this name in this yard")
        else: 
            break

    # Not sure what the rules for species will be
    species = input("Species: ");

    pet_payload = {"image" : 0, "name": name, "pet_yard": yard_uuid, "species": species} 

    try:
        requests.post(server + 'users/' + uuid + '/pets/new', verify=VERIFY_CERT, json=pet_payload, headers={'X-Auth-Key': user_token})
        print("Created pet " + name) 
    except ConnectionError: 
        print("Error when posting to server")
        return 1

def create_yard(server, user_content, uuid, user_token):
    while True:
        name = input("Yard name: ");
        # Need to check if there is another yard of the same name.
        if check_yard_name(server, name, user_content, uuid, user_token): 
            print("You already have a yard of this name")
            continue
        else:
            break
        
    yard_payload = {"image" : 0, "name": name} 
    try:
        requests.post(server + 'users/' + uuid + '/pet_yards/new', verify=VERIFY_CERT, json=yard_payload, headers={'X-Auth-Key': user_token})
        print("Created yard " + name)
    except ConnectionError: 
        print("Error when posting to server")
        return 1

# ==============================Deletion Funcitons==================================
