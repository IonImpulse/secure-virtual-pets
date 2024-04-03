#!/usr/bin/env python

import json
import requests 
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

def user_menu(username, user_content, uuid, user_token):

    print("Welcome " + username + ": What would you like to do?\n")
    user_command_list()
    #Prints out user command list
    while True:
        dec = input('> ')
        dec = dec.rstrip()
        if dec == '1':
            user_functions.view_pets(server, user_content, uuid, user_token)
        elif dec == '2': 
            pass 
        elif dec == '3': 
            pass
        elif dec == '4': 
            pass
        elif dec == 'quit':
            break
        else:
            print("I'm sorry, I didn't recognize that command.")


def login():
    username = input("Username: ");
    password = maskpass.askpass(prompt="Password: ")

    login_payload = {"password": password, "username": username} 
    response = requests.post(server + 'auth/login', verify=VERIFY_CERT, json=login_payload)
    
    if response.status_code == 200:
        print("Successfully logged in as " + username)
    else: 
        print("Login failed") 
        return response.status_code

    user_details = response.json() 

    # print(user_details) 
    user_token = user_details["token"]
    uuid = user_details["uuid"] 

    user_menu(username, user_details, uuid, user_token) 

def signup():
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

    signup_payload = { "email": email, "password": password, "username": username }
    response = requests.post(server + 'auth/signup', verify=VERIFY_CERT, json=signup_payload)
    print(response)

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
    [\033[32m2\033[0m] : View Yards
    [\033[32m3\033[0m] : Make a Pet 
    [\033[32m4\033[0m] : Do Something Else
    logout : close the program
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