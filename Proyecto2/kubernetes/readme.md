Para ver el external ip de nginx

kubectl get services -n nginx-ingress

Para mi caso me devolvio (pude cambiar si se rehace el procedimiento)
nginx-ingress-ingress-nginx-controller             LoadBalancer   34.118.235.114   35.239.71.57   80:31076/TCP,443:30721/TCP   21m

Para ver ruta para locust
kubectl describe ingress sopes1-ingress -n sopes1