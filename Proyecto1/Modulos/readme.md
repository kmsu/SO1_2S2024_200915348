COMANDOS

** Ejecutar desde la carpeta de modulos

make # crear compilados
sudo insmod <name>.ko # instalar modulo
sudo dmesg | tail -n 20 # ver logs del Kernel
sudo rmmod <name> # desinstalar modulo 

cat /proc/<PROC_NAME> # imprimir. En este caso sysinfo_200915348