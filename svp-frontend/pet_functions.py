import os
import random

path = os.path.dirname(os.path.abspath(__file__))

VERIFY_CERT = path + "/../svp-backend/cert.pem"

def print_petsheet(petsheet): 

    with open('pet_art/' + petsheet) as f_obj:

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

    with open('pet_art/' + petsheet) as f_obj:

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

        return pet
        #randsep will be a random "separator" value, represented as a random valid index for the separator list


# print_petsheet('dogs.txt')
print(get_random_pet_art('dogs.txt'))

