#!/usr/bin/env python
import requests 
from requests.exceptions import ConnectionError
import os 
import argparse
import re 

import maskpass

global server

import user_functions

path = os.path.dirname(os.path.abspath(__file__))

DEFAULT_PORT = 3000

DEFAULT_SERVER = f"https://localhost:{DEFAULT_PORT}/"

VERIFY_CERT = path + "/../svp-backend/cert.pem"

EMAIL_REGEX = r'\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,7}\b'

class App:
    def __init__(self, server):
        self.server = server

    def run(self):
        main_menu()

def check_email(email):
    if(re.fullmatch(EMAIL_REGEX, email)):
        return True
    else:
        return False

def main_menu():

    header()

    print("Secure Virtual Pets\n")

    command_list()
    #Prints out initial command list
    while True:

        dec = input('> ')

        dec = dec.rstrip()

        if dec == '1':
            login()
        elif dec == '2': 
            signup() 
        elif dec == 'quit':
            break
        else:
            print("I'm sorry, I didn't recognize that command.")

def user_menu(username, uuid, user_token):
    
    print("Welcome " + username + ": What would you like to do?\n")
    user_command_list()
    #Prints out user command list
    while True:
        response = requests.get(server + 'users/' + uuid, verify=VERIFY_CERT, headers={'X-Auth-Key': user_token})
        user_content = response.json()
        # print(user_content)
        dec = input('> ')
        dec = dec.rstrip()
        #View available pets 
        if dec == '1':
            user_functions.view_pets(server, user_content, uuid, user_token)
        #View Joined Yards
        elif dec == '2': 
            user_functions.view_joined_yards(server, user_content, uuid, user_token)
        #View your yards 
        elif dec == '3': 
            user_functions.view_owned_yards(server, user_content, uuid, user_token)
        elif dec == '4': 
            user_functions.create_pet(server, user_content, uuid, user_token)
        elif dec == '5': 
            user_functions.create_yard(server, user_content, uuid, user_token)
        elif dec == '6': 
            user_functions.delete_pet(server, user_content, uuid, user_token)
        elif dec == '7': 
            user_functions.delete_yard(server, user_content, uuid, user_token)
        elif dec == '8': 
            user_functions.manage_account(server, user_content, uuid, user_token)
        elif dec == 'logout':
            break
        else:
            print("I'm sorry, I didn't recognize that command.")


def login():
    try: 
        username = input("Username: ");
        password = maskpass.askpass(prompt="Password: ")
    except KeyboardInterrupt:
        print('\nAction Canceled')
        return

    login_payload = {"password": password, "username": username} 
    try:
        response = requests.post(server + 'auth/login', verify=VERIFY_CERT, json=login_payload)
    except ConnectionError: 
        print("Connection was refused")
        return 1
    
    if response.status_code == 200:
        print("Successfully logged in as " + username)
    else: 
        print("Login failed") 
        return response.status_code

    user_details = response.json() 

    # print(user_details) 
    user_token = user_details["token"]
    uuid = user_details["uuid"] 

    user_menu(username, uuid, user_token) 

def signup():
    try:

        if testing != 'True': 
            while True:
                email = input("Your Email: ");
                if check_email(email):
                    break
                else:
                    print("Invalid email. Please enter a vaild email.")
        else: 
            email = input("Your Email: ");

        username = input("Username: ");
        password = maskpass.askpass(prompt="Password: ")
    except KeyboardInterrupt:
        print('\nAction Canceled...')
        return

    signup_payload = { "email": email, "password": password, "username": username }
    response = requests.post(server + 'auth/signup', verify=VERIFY_CERT, json=signup_payload)

    if response.status_code == 200:
        print("Successfully signed up to server: " + server)
    else: 
        print("signup failed") 
        return response.status_code

def header():
    print(r"""
 $$$$$$\  $$\    $$\ $$$$$$$\  
$$  __$$\ $$ |   $$ |$$  __$$\ 
$$ /  \__|$$ |   $$ |$$ |  $$ |
\$$$$$$\  \$$\  $$  |$$$$$$$  |
 \____$$\  \$$\$$  / $$  ____/ 
$$\   $$ |  \$$$  /  $$ |      
\$$$$$$  |   \$  /   $$ |      
 \______/     \_/    \__|      
 """)
                               
#header. Prints the nice header. ASCII art generated by patorjk's TAAG

def command_list():
    print("""
    [\033[32m1\033[0m] : Login
    [\033[32m2\033[0m] : Signup 
    quit : close the program
    """)


def user_command_list():
    print("""
    [\033[32m1\033[0m] : View Pets
    [\033[32m2\033[0m] : View Joined Yards
    [\033[32m3\033[0m] : View Owned Yards 
    [\033[32m4\033[0m] : Create a Pet
    [\033[32m5\033[0m] : Create a Yard 
    [\033[32m6\033[0m] : Delete a Pet
    [\033[32m7\033[0m] : Delete a Yard 
    [\033[32m8\033[0m] : Manage Your Account 
    logout : logout of your user 
    """)

if __name__ == "__main__": 
    parser = argparse.ArgumentParser(description="Secure Virtual Pets")
    parser.add_argument("--server", default=DEFAULT_SERVER, help="Server URL")
    parser.add_argument("--testing", default='True', help="Server URL")
    args = parser.parse_args()
    server = args.server
    testing = args.testing
    app = App(server)
    app.run()
