package com.aoc.Day08;

import com.aoc.Day;

import java.util.*;

public class Day08 implements Day {
    @Override
    public String part1(List<String> input) {
        List<List<String>> rows = parseInput(input);
        List<List<String>> sortedLastRows = getSortedArray(rows);

        Map<String, Integer> stringIterations = new HashMap<>();
        for(List<String> i : sortedLastRows) {
            for(String j : i) {
                if(j.length() == 3 || j.length() == 2 || j.length() == 4 || j.length() == 7) {
                    if (!stringIterations.containsKey(j)) stringIterations.put(j, 0);
                    stringIterations.put(j, stringIterations.get(j) + 1);
                }
            }
        }
        int cpt = 0;
        for(String i : stringIterations.keySet()){
            cpt += stringIterations.get(i);
        }
        return cpt + "";
    }

    private List<List<String>> parseInput(List<String> input) {
        List<List<String>> rows = new ArrayList<>();
        for(String i : input) {
            rows.add(Arrays.asList(i.split(" \\| ")));
        }
        return rows;
    }

    private List<List<String>> getSortedArray(List<List<String>> rows) {
        List<List<String>> sortedArray = new ArrayList<>();
        for(List<String> i : rows) {
            List<String> sortedRow = new ArrayList<>();
            String[] tmp = i.get(1).split(" ");
            for(String j : tmp) {
                char[] tmpArr = j.toCharArray();
                Arrays.sort(tmpArr);
                sortedRow.add(new String(tmpArr));
            }
            sortedArray.add(sortedRow);
        }
        return sortedArray;
    }

    @Override
    public String part2(List<String> input) {
        return null;
    }
}
