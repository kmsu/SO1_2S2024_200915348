# Dockerfile para un contenedor de bajo consumo
FROM python:3.9-slim

RUN pip install flask

COPY low2.py /low2.py

CMD ["python", "/low2.py"]