import requests
import os
path = os.path.dirname(os.path.abspath(__file__))

VERIFY_CERT = path + "/../svp-backend/cert.pem"


def view_pets(server, user_content, uuid , user_token):
    for pet_uuid in user_content["pets"]:
        response = requests.get(server + 'users/' + uuid + '/pets/' + pet_uuid, verify=VERIFY_CERT, headers={'X-Auth-Key': user_token})
        print(pet_uuid, response)

def view_joined_yards(server, user_content, uuid , user_token):
    print(user_content)
    for pet_yard_uuid in user_content["joined_pet_yards"]:
        response = requests.get(server + 'users/' + uuid + '/pets_yards/' + pet_yard_uuid, verify=VERIFY_CERT, headers={'X-Auth-Key': user_token})
        print(pet_yard_uuid, response)

def view_owned_yards(server, user_content, uuid , user_token):
    for yard_uuid in user_content['owned_pet_yards']:
        response = requests.get(server + 'users/' + uuid + '/pet_yards/' + yard_uuid, verify=VERIFY_CERT, headers={'X-Auth-Key': user_token})
        response_content = response.json()
        print(response_content["name"])

def create_pet(server, user_content, uuid, user_token):

    name = input("Pet name: ");

    # Need to check if there is another pet of the same name in the yard.
    yard = input("Pet yard: ");
    species = input("Species: ");

    pet_payload = {"image" : 0, "name": name, "pet_yard": yard, "species": species} 

    try:
        response = requests.post(server + 'users/' + uuid + '/pets/new', verify=VERIFY_CERT, json=pet_payload, headers={'X-Auth-Key': user_token})
    except ConnectionError: 
        print("Error when posting to server")
        return 1


def check_yard_name(server, name, user_content, uuid, user_token): 
    for yard_uuid in user_content['owned_pet_yards']:
        response = requests.get(server + 'users/' + uuid + '/pet_yards/' + yard_uuid, verify=VERIFY_CERT, headers={'X-Auth-Key': user_token})
        response_content = response.json()
        if name == response_content['name']:
            print("You already have a yard of this name")
            return True
    return False

def create_yard(server, user_content, uuid, user_token):
    while True:
        name = input("Yard name: ");
        # Need to check if there is another yard of the same name.
        if check_yard_name(server, name, user_content, uuid, user_token): 
            continue
        else:
            break
        
    yard_payload = {"image" : 0, "name": name} 
    try:
        response = requests.post(server + 'users/' + uuid + '/pet_yards/new', verify=VERIFY_CERT, json=yard_payload, headers={'X-Auth-Key': user_token})
        print("Created yard " + name)
    except ConnectionError: 
        print("Error when posting to server")
        return 1



