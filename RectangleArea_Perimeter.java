package Inventyv_Training;
import java.util.*;

public class RectangleArea_Perimeter {

	public static void main(String[] args) {
		Scanner sc=new Scanner(System.in);
		System.out.println("Please enter length and breath");
		int length = sc.nextInt();
		int breath = sc.nextInt();
		
		int area = length*breath;
		int perimeter = 2*length*breath;
		System.out.println("Area : "+area+"and Perimeter is : "+perimeter);

	}

}
