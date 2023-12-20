import java.util.Scanner;

public class maxOf2 {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        
        System.out.print("Enter x: ");
        int x = scanner.nextInt();
        
        System.out.print("Enter y: ");
        int y = scanner.nextInt();
        
        if (x >= y) {
            System.out.println(x);
        }
    }
}