package test.aoc.Day07;

import com.aoc.Day07.Day07;
import org.junit.Test;

import java.util.List;

import static org.junit.Assert.assertEquals;

public class Day07Test {
    @Test
    public void testPart1(){
        // Given
        List<String> input = List.of("16,1,2,0,4,2,7,1,2,14");

        // When
        String result = new Day07().part1(input);

        // Then
        assertEquals("37", result);
    }

    @Test
    public void testPart2(){
        List<String> input = List.of("16,1,2,0,4,2,7,1,2,14");

        // When
        String result = new Day07().part2(input);

        // Then
        assertEquals("26984457539", result);
    }
}
