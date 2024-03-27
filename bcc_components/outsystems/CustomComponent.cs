using System;
using System.Text.RegularExpressions;

namespace CustomComponents
{
    public class CustomCalculator
    {
        // Método para somar dois números
        public int Add(int a, int b)
        {
            return a + b;
        }

        // Método para subtrair dois números
        public int Subtract(int a, int b)
        {
            return a - b;
        }

        // Método para multiplicar dois números
        public int Multiply(int a, int b)
        {
            return a * b;
        }

        // Método para dividir dois números
        public double Divide(int a, int b)
        {
            if (b == 0)
            {
                // Tratamento para divisão por zero
                throw new ArgumentException("Divisor cannot be zero.");
            }

            return (double)a / b;
        }

        // Método para validar NIF (Número de Identificação Fiscal)
        public bool ValidateNIF(string nif)
        {
            // NIF deve conter 9 dígitos
            if (!Regex.IsMatch(nif, @"^\d{9}$"))
            {
                return false;
            }

            // Algoritmo de validação do NIF
            int[] weights = { 9, 8, 7, 6, 5, 4, 3, 2, 1 };
            int sum = 0;
            for (int i = 0; i < 9; i++)
            {
                sum += int.Parse(nif[i].ToString()) * weights[i];
            }

            int remainder = sum % 11;
            int checkDigit = remainder > 1 ? 11 - remainder : 0;

            return int.Parse(nif[8].ToString()) == checkDigit;
        }

        // Método para validar CPF (Cadastro de Pessoas Físicas)
        public bool ValidateCPF(string cpf)
        {
            // CPF deve conter 11 dígitos
            if (!Regex.IsMatch(cpf, @"^\d{11}$"))
            {
                return false;
            }

            // Algoritmo de validação do CPF
            int[] weights = { 10, 9, 8, 7, 6, 5, 4, 3, 2 };
            int sum = 0;
            for (int i = 0; i < 9; i++)
            {
                sum += int.Parse(cpf[i].ToString()) * weights[i];
            }

            int remainder = sum % 11;
            int checkDigit1 = remainder < 2 ? 0 : 11 - remainder;

            if (int.Parse(cpf[9].ToString()) != checkDigit1)
            {
                return false;
            }

            sum = 0;
            for (int i = 0; i < 10; i++)
            {
                sum += int.Parse(cpf[i].ToString()) * weights[i];
            }

            remainder = sum % 11;
            int checkDigit2 = remainder < 2 ? 0 : 11 - remainder;

            return int.Parse(cpf[10].ToString()) == checkDigit2;
        }
    }
}
