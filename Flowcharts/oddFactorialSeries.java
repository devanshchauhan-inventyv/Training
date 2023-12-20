import java.util.Scanner;

public class oddFactorialSeries {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        
        int i = 3;
        int fact = 1;
        
        System.out.print("Enter a number: ");
        int n = scanner.nextInt();
        
        while (i <= ((n * n) / 2)) {
            System.out.println(fact);
            
            fact = fact * i * (i - 1) * (-1);
            i = i + 2;
        }
        
        scanner.close();
    }
}
