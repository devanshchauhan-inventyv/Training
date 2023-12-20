import java.util.Scanner;

public class sumOfn {
    public static void main(String[] args) {
        Scanner input = new Scanner(System.in);
        System.out.print("Enter a number: ");
        int n = input.nextInt();
        
        int i = 1;
        int sum = 0;
        
        while (i <= n) {
            sum += i;
            i++;
        }
        
        System.out.println(sum);
    }
}
