import java.util.Scanner;

public class sineSeries {
    public static void main(String[] args) {
        Scanner input = new Scanner(System.in);
        double x, sum, t;
        int n, i;
        
        n = input.nextInt();
        
        x = input.nextDouble();
        x = (x * 3.14159) / 180;
        
        t = x;
        sum = x;
        i = 1;
        
        while (i <= n) {
            t = t * (-1) * x * x / (2 * i * (2 * i + 1));
            sum = sum + t;
            i = i + 1;
        }
        
        System.out.println(sum);
    }
}