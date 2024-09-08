# Script de bajo consumo usando Flask
from flask import Flask

app = Flask(__name__)

@app.route('/')
def hello_world():
    return 'Nuevo hola mundo, segundo bajo consumo'

if __name__ == '__main__':
    app.run(host='0.0.0.0')