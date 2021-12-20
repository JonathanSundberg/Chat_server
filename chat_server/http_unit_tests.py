import requests

def register_user():
    url = "http://localhost:8000/message/register"
    test_user = {
      "user_name": "Python_test_user",
      "password": "Python_test_password",
      "email": "Python_test_emaill@trying.com",
      "id": ""
    }
    post_request = requests.post(url, json=test_user)
    print(type(post_request.status_code))
    assert post_request.status_code == 200, "Failed to register user"

def remove_user_from_database():
    url = "http://localhost:8000/message/remove"
    test_user = {
      "user_name": "Python_test_user",
      "password": "Python_test_password",
      "email": "Python_test_emaill@trying.com",
      "id": ""
    }
    post_request = requests.post(url, json=test_user)
    print(type(post_request.status_code))
    assert post_request.status_code == 200, "Failed to remove user"



def main():
    register_user()
    #remove_user_from_database()
    

if __name__ == "__main__":
    main()