package test.aoc.Day03;

import com.aoc.Day03.Day03;
import org.junit.Test;

import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import static org.junit.Assert.assertEquals;


public class Day03Test {
    @Test
    public void testPart1(){
        // Given
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
        String result = new Day03().part1(input);

        // Then
        assertEquals("198", result);
    }

    @Test
    public void testPart2(){
        // Given
        List<String> input = Collections.singletonList("test");

        // When
        String result = new Day03().part2(input);

        // Then
        assertEquals(input.get(0), result);
    }
}
