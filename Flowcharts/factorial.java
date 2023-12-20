import java.util.Scanner;

public class factorial {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        int n, i = 1, result = 1;

        System.out.println("Enter a number: ");
        n = scanner.nextInt();

        while (i <= n) {
            result *= i;
            i++;
        }

        System.out.println("Factorial of " + n + " is: " + result);
    }
}
