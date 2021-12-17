package test.aoc.Day06;

import com.aoc.Day06.Day06;
import org.junit.Test;

import java.util.Arrays;
import java.util.List;

import static org.junit.Assert.assertEquals;

public class Day06Test {
    @Test
    public void testPart1(){
        // Given
        List<String> input = Arrays.asList("3","4","3","1","2");

        // When
        String result = new Day06().part1(input);

        // Then
        assertEquals("5934", result);
    }

    @Test
    public void testPart2(){
        List<String> input = Arrays.asList(
               ""
        );

        // When
        String result = new Day06().part2(input);

        // Then
        assertEquals("12", result);
    }
}
