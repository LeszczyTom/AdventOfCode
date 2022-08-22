package test.aoc.Day05;

import com.aoc.Day05.Day05;
import org.junit.Test;

import java.util.Arrays;
import java.util.List;

import static org.junit.Assert.assertEquals;

public class Day05Test {
    @Test
    public void testPart1(){
        // Given
        List<String> input = Arrays.asList(
                "0,9 -> 5,9",
                "8,0 -> 0,8",
                "9,4 -> 3,4",
                "2,2 -> 2,1",
                "7,0 -> 7,4",
                "6,4 -> 2,0",
                "0,9 -> 2,9",
                "3,4 -> 1,4",
                "0,0 -> 8,8",
                "5,5 -> 8,2"
        );

        // When
        String result = new Day05().part1(input);

        // Then
        assertEquals("5", result);
    }

    @Test
    public void testPart2(){
        List<String> input = Arrays.asList(
                "0,9 -> 5,9",
                "8,0 -> 0,8",
                "9,4 -> 3,4",
                "2,2 -> 2,1",
                "7,0 -> 7,4",
                "6,4 -> 2,0",
                "0,9 -> 2,9",
                "3,4 -> 1,4",
                "0,0 -> 8,8",
                "5,5 -> 8,2"
        );

        // When
        String result = new Day05().part2(input);

        // Then
        assertEquals("12", result);
    }
}
