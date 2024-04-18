import requests
import os
path = os.path.dirname(os.path.abspath(__file__))

VERIFY_CERT = path + "/../svp-backend/cert.pem"


#TODO: Control C on the menu's should probably just log you out / exit the application instead of crashing
#      Ok creating a pet still doesn't place it in the yard, gotta do that

# ==============================Viewing Funcitons==================================
def view_pets(server, user_content, uuid , user_token):
    for pet_uuid in user_content["pets"]:

        response = requests.get(server + 'users/' + uuid + '/pets/' + pet_uuid, verify=VERIFY_CERT, headers={'X-Auth-Key': user_token})
        response_content = response.json()
        pet_name = response_content['name']
        pet_species = response_content['species']
        pet_level = response_content['level']
        yard_uuid = response_content['pet_yard']

        #This can fail if an incorrect uuid is passed, but this should never occur.

        response = requests.get(server + 'users/' + uuid + '/pet_yards/' + yard_uuid, verify=VERIFY_CERT, headers={'X-Auth-Key': user_token})
        response_content = response.json()
        yard_name = response_content['name']

        print("Pet: " + pet_name + " Species: " + pet_species + " Level: " + str(pet_level) + " Yard: " + yard_name)

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

def check_pet_name_in_yard(server, pet_name, yard_uuid, uuid, user_token): 
    
    response = requests.get(server + 'users/' + uuid + '/pet_yards/' + yard_uuid, verify=VERIFY_CERT, headers={'X-Auth-Key': user_token})
    response_content = response.json()
    for pet_uuid in response_content['pets']:
        response = requests.get(server + 'users/' + uuid + '/pets/' + pet_uuid, verify=VERIFY_CERT, headers={'X-Auth-Key': user_token})
        response_content = response.json()
        if response_content['name'] == pet_name:
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
    if len(user_content['owned_pet_yards']) == 0:
        print("You have no pet yards")
        return 
    yard_name = ""
    yard_uuid = ""
    name = ""
    species = ""
    try:
        while True:
            yard_name = input("Pet yard: ");
            if check_yard_name(server, yard_name, user_content, uuid, user_token):
                yard_uuid = check_yard_name(server, yard_name, user_content, uuid, user_token)
                break
            else:
                print("There is no yard with this name")
        while True:
            name = input("Pet name: ");
            if check_pet_name_in_yard(server, name, yard_uuid, uuid, user_token):
                print("There is already a pet with this name in this yard")
            elif len(name) == 0: 
                print("Pet's must have a name")
            else: 
                break
        # Not sure what the rules for species will be
        species = input("Species: ");
    except KeyboardInterrupt: 
        print('\nAction Canceled...')
        return

    pet_payload = {"image" : 0, "name": name, "pet_yard": yard_uuid, "species": species} 

    try:
        response = requests.post(server + 'users/' + uuid + '/pets/new', verify=VERIFY_CERT, json=pet_payload, headers={'X-Auth-Key': user_token})
        if response.status_code == 200:
            pass
        else: 
            print("Failed to make pet failed") 
            print(response.status_code)
            return 
        response_content = response.json()
        pet_uuid = response_content['uuid']
        response = requests.patch(server + 'users/' + uuid + '/pet_yards/' + yard_uuid + '/pet/' + pet_uuid, verify=VERIFY_CERT, headers={'X-Auth-Key': user_token})
        if response.status_code == 200:
            pass
        else: 
            print("Failed to patch pet into yard, pet not created") 
            print(response.status_code)
            print(server + 'users/' + uuid + '/pet_yards/' + yard_uuid + '/pet/' + pet_uuid)
            requests.delete(server + 'users/' + uuid + '/pets/' + pet_uuid, verify=VERIFY_CERT, json=pet_payload, headers={'X-Auth-Key': user_token})
            return 

        print("Created pet " + name + " in yard " + yard_name ) 
        return response.status_code
    except ConnectionError: 
        print("Error when posting to server")
        return 1

def create_yard(server, user_content, uuid, user_token):
    try:
        while True:
            name = input("Yard name: ");
            # Need to check if there is another yard of the same name.
            if check_yard_name(server, name, user_content, uuid, user_token): 
                print("You already have a yard of this name")
            elif len(name) == 0:
                print("Yard;s must have a name")
            else:
                break
    except KeyboardInterrupt:
        print('\nAction Canceled...')
        return
        
    yard_payload = {"image" : 0, "name": name} 
    try:
        requests.post(server + 'users/' + uuid + '/pet_yards/new', verify=VERIFY_CERT, json=yard_payload, headers={'X-Auth-Key': user_token})
        print("Created yard " + name)
    except ConnectionError: 
        print("Error when posting to server")
        return 1

# ==============================Deletion Funcitons==================================

def delete_pet(server, user_content, uuid, user_token):
    pet_name = "" 
    pet_uuid = "" 
    try:
        print("Warning! This will permanently delete a pet. Would you like to proceed?")
        choice = input("Proceed? (Yes/No) ")
        if choice[0].lower() == 'y': 

            while True:
                yard_name = input("Name of yard the pet is in: ")
                if check_yard_name(server, yard_name, user_content, uuid, user_token):
                    yard_uuid = check_yard_name(server, yard_name, user_content, uuid, user_token)
                    break
                else:
                    print("There is no yard with this name")

            while True:
                pet_name = input("Pet name: ");
                if check_pet_name_in_yard(server, pet_name, yard_uuid, uuid, user_token):
                    pet_uuid = check_pet_name_in_yard(server, pet_name, yard_uuid, uuid, user_token)
                    break
                else:
                    print(check_pet_name_in_yard(server, pet_name, yard_uuid, uuid, user_token))
                    print("No pet with this name in yard " + yard_name)
    except KeyboardInterrupt: 
        print("Action Canceled . . .\n")
        return

    print("Deleting " + pet_name + " . . . ")
    response = requests.delete(server + 'users/' + uuid + '/pets/' + pet_uuid , verify=VERIFY_CERT, headers={'X-Auth-Key': user_token})

    if response.status_code == 200:
        print("Successfully deleted pet " + pet_name)
    else: 
        print("Deletion failed") 
        return response.status_code


def delete_yard(server, user_content, uuid, user_token):
    yard_name = ""
    yard_uuid = ""
    try:
        print("Warning! This will permanently delete a yard, including all pets within it. Would you like to proceed?")
        choice = input("Proceed? (Yes/No) ")
        if choice[0].lower() == 'y': 
            while True:
                yard_name = input("Name of yard: ")
                if check_yard_name(server, yard_name, user_content, uuid, user_token):
                    yard_uuid = check_yard_name(server, yard_name, user_content, uuid, user_token)
                    break
                else:
                    print("There is no yard with this name")
        else: 
            return
    except KeyboardInterrupt:
        print("Action Canceled...\n")
        return

    print("Deleting " + yard_name + " . . . ")
    response = requests.get(server + 'users/' + uuid + '/pet_yards/' + yard_uuid, verify=VERIFY_CERT, headers={'X-Auth-Key': user_token})
    response_content = response.json()

    for pet_uuid in response_content['pets']:
        response = requests.delete(server + 'users/' + uuid + '/pets/' + pet_uuid, verify=VERIFY_CERT, headers={'X-Auth-Key': user_token})
    
    response = requests.delete(server + 'users/' + uuid + '/pet_yards/' + yard_uuid, verify=VERIFY_CERT, headers={'X-Auth-Key': user_token})

    if response.status_code == 200:
        print("Yard successfully deleted")
    else: 
        print("Error with deleting yard")
        print(response.status_code)

def delete_user(server, uuid, user_token):
    global quitflag 
    print("Warning! This will delete your account, including all of your pet's and yard's, and close the program! The account will not be recoverable after you complete this action")
    choice = input("Proceed? (Yes/No) ")
    if choice[0].lower() == 'y': 
        requests.delete(server + 'users/' + uuid, verify=VERIFY_CERT, headers={'X-Auth-Key': user_token})
        exit(0)
    else: 
        return

# ==============================Account Management==================================

def manage_account(server, user_content, uuid, user_token): 
    manage_account_command_list()
    #Prints out management list
    while True:
        response = requests.get(server + 'users/' + uuid, verify=VERIFY_CERT, headers={'X-Auth-Key': user_token})
        user_content = response.json()
        # print(user_content)
        dec = input('> ')
        dec = dec.rstrip()
        #View available pets 
        if dec == '1':
            delete_user(server, uuid, user_token)
        #View Joined Yards
        elif dec == '2': 
            break
        else:
            print("I'm sorry, I didn't recognize that command.")
    pass

def manage_account_command_list():
    print("""
    [\033[32m1\033[0m] : Delete Account
    [\033[32m2\033[0m] : Back
    """)
