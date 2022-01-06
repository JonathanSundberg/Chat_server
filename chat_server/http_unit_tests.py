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


def create_conversation():
    url = "http://localhost:8000/conversation/create_conversation"
    test_user = {
    "user_id": "Python_user_1",
    "users_to_invite": ["python_user_2"],
    "public": False
    }
    post_request = requests.post(url, json=test_user)
    print(post_request.status_code)
    assert post_request.status_code == 200, "Failed to create conversation"



def send_message():
  url = "http://localhost:8000/message/received"



def main():
    register_user()
    #remove_user_from_database()
    

if __name__ == "__main__":
    main()