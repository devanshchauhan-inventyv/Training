import java.util.Scanner;

public class series3 {
    public static void main(String[] args) {
        Scanner input = new Scanner(System.in);
        int i = 1, n, j = 1;

        System.out.print("Enter the value of n: ");
        n = input.nextInt();

        if (i <= n) {
            j = i;

            System.out.println(j);

            j++;
            j = i - 1;

            System.out.println(j);

            j = 1;

            while (j >= 1) {
                j--;

                // continue with the rest of your code
            }
        }
    }
}
