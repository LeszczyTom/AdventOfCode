package test.aoc.Day03;

import com.aoc.Day04.Day04;
import org.junit.Test;

import java.util.Arrays;
import java.util.List;

import static org.junit.Assert.assertEquals;

public class Day04Test {
    @Test
    public void testPart1(){
        // Given
        List<String> input = Arrays.asList(
                "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
                "",
                "22 13 17 11  0",
                "8  2 23  4 24",
                "21  9 14 16  7",
                "6 10  3 18  5",
                "1 12 20 15 19",
                "",
                "3 15  0  2 22",
                "9 18 13 17  5",
                "19  8  7 25 23",
                "20 11 10 24  4",
                "14 21 16 12  6",
                "",
                "14 21 17 24  4",
                "10 16 15  9 19",
                "18  8 23 26 20",
                "22 11 13  6  5",
                "2  0 12  3  7"
        );

        // When
        String result = new Day04().part1(input);

        // Then
        assertEquals("4512", result);
    }

    @Test
    public void testPart2(){
        List<String> input = Arrays.asList(
                "00100",
                "11110",
                "10110",
                "10111",
                "10101",
                "01111",
                "00111",
                "11100",
                "10000",
                "11001",
                "00010",
                "01010"
        );

        // When
        String result = new Day04().part2(input);

        // Then
        assertEquals("230", result);
    }
}
