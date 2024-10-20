from locust import HttpUser, task

class LoadTest(HttpUser):
    @task
    def send_data(self):
        data = {
            "student": "Kevin Samayoa",
            "age": 20,
            "faculty": "Ingenieria",
            "discipline": 1
        }
        self.client.post("/", json=data)
