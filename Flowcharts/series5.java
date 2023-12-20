import java.util.Scanner;

public class series5 {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        int i = 1;
        int n = scanner.nextInt();
        int j = 1;

        if (i <= n) {
            j = i * 2 - 1;
            System.out.println(j % 10);
            j++;
            j = i * 2 - 1;
            i++;
            System.out.println(j % 10);
        }
        while (j > i) {
            j--;
        }
        scanner.close();
    }
}
