import java.util.Scanner;

public class series6 {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        
        int i = 0, j = 1, n, ans = 1, k = 1;
        
        System.out.print("Enter the value of n: ");
        n = scanner.nextInt();

        while (j < n * 2) {
            System.out.println(ans);
            System.out.println(j);
            
            ans = ans + i;
            i = k;
            k = ans;
            j += 2;
        }
        
        scanner.close();
    }
}
