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

def password_check(password):
    """
    Verify the strength of 'password'
    Returns a dict indicating the wrong criteria
    A password is considered strong if:
        8 characters length or more
        1 digit or more
        1 symbol or more
        1 uppercase letter or more
        1 lowercase letter or more
    """

    # calculating the length
    length_error = len(password) < 8

    # searching for digits
    digit_error = re.search(r"\d", password) is None

    # searching for uppercase
    uppercase_error = re.search(r"[A-Z]", password) is None

    # searching for lowercase
    lowercase_error = re.search(r"[a-z]", password) is None

    # searching for symbols
    symbol_error = re.search(r"[ !#$%&'()*+,-./[\\\]^_`{|}~"+r'"]', password) is None

    # overall result
    password_ok = not ( length_error or digit_error or uppercase_error or lowercase_error or symbol_error )

    return {
        'password_ok' : password_ok,
        'length_error' : length_error,
        'digit_error' : digit_error,
        'uppercase_error' : uppercase_error,
        'lowercase_error' : lowercase_error,
        'symbol_error' : symbol_error,
    }

def main_menu():

    header()

    print("Secure Virtual Pets\n")

    command_list()
    #Prints out initial command list
    try: 
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
    except KeyboardInterrupt:
        print("Exiting...") 
        exit(0)

def user_menu(username, uuid, user_token):
    
    print("Welcome " + username + ": What would you like to do?\n")
    user_command_list()
    functions = [
        user_functions.view_pets,
        user_functions.view_joined_yards,
        user_functions.view_owned_yards,
        user_functions.create_pet,
        user_functions.create_yard,
        user_functions.delete_pet,
        user_functions.delete_yard,
        user_functions.manage_account
    ]

    while True:
        try:
            response = requests.get(server + 'users/' + uuid, verify=VERIFY_CERT, headers={'X-Auth-Key': user_token})
            user_content = response.json()

            dec = input('> ').rstrip()

            if dec == 'logout':
                break
            elif dec.isdigit() and int(dec) in range(1, len(functions) + 1):
                functions[int(dec) - 1](server, user_content, uuid, user_token)
            else:
                print("I'm sorry, I didn't recognize that command.")
        except KeyboardInterrupt:
            print("Logging out...")
            break


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

        if testing != 'True':
            while True:
                password = maskpass.askpass(prompt="Password: ")
                pass_result = password_check(password)['password_ok']
                if pass_result == False: 
                    print("Password must be longer then 8 characters, and contain a digit, a special character, and an uppercase character")
                else: 
                    break
        else:
            password = maskpass.askpass(prompt="Password: ")
    except KeyboardInterrupt:
        print('\nAction Canceled...')
        return

    signup_payload = { "email": email, "password": password, "username": username }

    try:
        response = requests.post(server + 'auth/signup', verify=VERIFY_CERT, json=signup_payload)
    except ConnectionError: 
        print("Connection was refused")
        return 1

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
