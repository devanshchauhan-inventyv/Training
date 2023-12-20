import java.util.Scanner;

public class series4 {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);

        System.out.print("Enter a value for n: ");
        int n = scanner.nextInt();

        for (int i = 1; i <= n; i++) {
            int j = i * 2 - 1;
            System.out.println(j);
            j++;
            j = i * 2 - 1;
            System.out.println(j);

            if (j > i + 1) {
                j--;
            }
        }

        scanner.close();
    }
}