import os
import random
import requests 

path = os.path.dirname(os.path.abspath(__file__))

VERIFY_CERT = path + "/../svp-backend/cert.pem"

import user_functions

def print_petsheet(petsheet): 
    
    with open(path + '/pet_art/' + petsheet) as f_obj:

        petsheet = f_obj.readlines()
        separators = [] 
        line_number = 1
        for line in petsheet:
            if len(line) == 1: 
                separators.append(line_number)
            line_number += 1 

        for value in separators: 
            c = 0
            while True:
                try:
                    if len(petsheet[value + c]) == 1: 
                        print('##################################')
                        break
                    else:
                        print(petsheet[value + c], end = '')
                        c += 1 
                except IndexError: 
                    break

def get_random_pet_art(petsheet): 

    with open(path + '/pet_art/' + petsheet) as f_obj:

        petsheet = f_obj.readlines()
        separators = [] 
        line_number = 1

        for line in petsheet:
            if len(line) == 1: 
                separators.append(line_number)
            line_number += 1 

        randsep = separators[random.randrange(len(separators))]
        c = 0
        pet = '' 
        while True:
                try:
                    if len(petsheet[randsep + c]) == 1: 
                        break
                    else:
                        pet += petsheet[randsep + c]
                        c += 1 
                except IndexError: 
                    break

        print(pet)
        return randsep


def get_pet_art(petsheet, index): 

    with open(path + '/pet_art/' + petsheet) as f_obj:

        petsheet = f_obj.readlines()
        separators = [] 
        line_number = 1

        for line in petsheet:
            if len(line) == 1: 
                separators.append(line_number)
            line_number += 1 

        c = 0
        pet = '' 
        while True:
                try:
                    if len(petsheet[index + c]) == 1: 
                        break
                    else:
                        pet += petsheet[index + c]
                        c += 1 
                except IndexError: 
                    break

        return pet

        #randsep will be a random "separator" value, represented as a random valid index for the separator list

# It works here...something to do with request causing the json parsing to go screwy?
# I'll just store an index for now and try to redo the art storage later

# print_petsheet('dog.txt')
# image = get_random_pet_art('dog.txt')
# payload = {'image': image}
# payload = rf"{image}"
# payload = {'image': rf"{image}"}

# payload = json.dumps(payload) 
# print(payload)

# payload = json.loads(payload)
# print(payload)
# print(payload['image'])


def feed_pet(server, uuid, user_token, pet_uuid): 
    response = requests.post(server + 'users/' + uuid + '/pets/' + pet_uuid + '/feed', verify=VERIFY_CERT, headers={'X-Auth-Key': user_token})
   
def feed_yard(server, user_content, uuid, user_token):
    yard_name = ""
    yard_uuid = ""
    try:
        while True:
            yard_name = input("Name of yard: ")
            if user_functions.check_yard_name(server, yard_name, user_content, uuid, user_token):
                yard_uuid = user_functions.check_yard_name(server, yard_name, user_content, uuid, user_token)
                break
            else:
                print("There is no yard with this name")
    except KeyboardInterrupt:
        print("Action Canceled...\n")
        return

    print("Feeding all pets in " + yard_name + " . . . ")
    response = requests.get(server + 'users/' + uuid + '/pet_yards/' + yard_uuid, verify=VERIFY_CERT, headers={'X-Auth-Key': user_token})
    response_content = response.json()

    for pet_uuid in response_content['pets']:
        feed_pet(server, uuid, user_token, pet_uuid)

    print("Yard successfully fed")
    
