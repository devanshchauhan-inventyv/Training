import java.util.Scanner;

public class series2 {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        
        System.out.print("Enter the value of n: ");
        int n = scanner.nextInt();
        
        int i = 1;
        int j = 1;
        
        if (i <= n) {
            j = 1;
            while (j <= i) {
                System.out.println(j);
                j++;
            }
            i++;
        }
    }
}