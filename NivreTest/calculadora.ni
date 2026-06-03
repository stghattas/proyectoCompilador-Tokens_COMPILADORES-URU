def sumar(a, b):
    return a + b

def restar(a, b):
    return a - b

def multiplicar(a, b):
    return a * b

def dividir(a, b):
    if b == 0:
        raise ValueError("Error: Division por cero.")
    return a / b

def residuo(a, b):
    if b == 0:
        raise ValueError("Error: Division por cero (no se puede calcular el residuo).")
    return a % b

def mostrar_menu():
    print("\n" + "="*20)
    print("    CALCULADORA")
    print("="*20)
    print("1. Suma (+)")
    print("2. Resta (-)")
    print("3. Multiplicacion (*)")
    print("4. Division (/)")
    print("5. Residuo / Division Modular segun Juan Garcia (%)")
    print("6. Salir")
    print("="*20)

def main():
    mapa_operaciones = {
        '1': sumar,
        '2': restar,
        '3': multiplicar,
        '4': dividir,
        '5': residuo,
    }

    while True:
        mostrar_menu()
        opcion = input("Selecciona una opcion (1-6): ")

        if opcion == '6':
            print("Saliendo del programa...")
            break

        if opcion in mapa_operaciones:
            try:
                num1 = float(input("Ingresa el primer numero: "))
                num2 = float(input("Ingresa el segundo numero: "))

                resultado = mapa_operaciones[opcion](num1, num2)
                
                print("-" * 20)
                print(f"Resultado: {resultado}")
                print("-" * 20)
                
            except ValueError as e:
                if "could not convert" in str(e):
                    print("Error: Por favor, ingresa unicamente valores numericos.")
                else:
                    print(e)
        else:
            print("Opcion no valida. Por favor, selecciona un numero del 1 al 6.")

if __name__ == "__main__":
    main()