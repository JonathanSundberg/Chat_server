import requests

def register_user():
    url = "http://localhost:8000/message/register"
    test_user = {
      "user_name": "Python_test_user",
      "password": "Python_test_password",
      "email": "Python_test_emaill@trying.com"
    }
    post_request = requests.post(url, data=test_user)





def main():
    register_user()

if __name__ == "__main__":
    main()