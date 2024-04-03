import requests
import os
path = os.path.dirname(os.path.abspath(__file__))

VERIFY_CERT = path + "/../svp-backend/cert.pem"


def view_pets(server, user_content, uuid , user_token):
    for pet_uuid in user_content["pets"]:
        response = requests.get(server + 'users/' + uuid + '/pets/' + pet_uuid, verify=VERIFY_CERT, headers={'X-Auth-Key': user_token})
        # print(pet_uuid, response)
