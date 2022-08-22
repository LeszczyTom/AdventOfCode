package com.aoc.Day07;

import com.aoc.Day;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class Day07 implements Day {
    List<Integer> horizontalPosition = new ArrayList<>();
    @Override
    public String part1(List<String> input) {
        List<String> inputList = List.of(input.get(0).split(","));
        for(String i : inputList) horizontalPosition.add(Integer.parseInt(i));

        System.out.println(horizontalPosition);

        long min = 999999999;
        int max = horizontalPosition.get(0);
        for(int i : horizontalPosition) {
            if(i > max) max = i;
        }
        for(int i = 0; i <= max; i++) {
            long tmp = costMoveTo(i);
            if(tmp < min) min = tmp;
        }
        return min + "";
    }

    private long costMoveTo(int pos) {
        long cpt = 0;
        for(int i : horizontalPosition) {
            if(i < pos) cpt += pos - i;
            if(i > pos) cpt += i - pos;
        }
        return cpt;
    }

    @Override
    public String part2(List<String> input) {
        List<String> inputList = List.of(input.get(0).split(","));
        for(String i : inputList) horizontalPosition.add(Integer.parseInt(i));

        System.out.println(horizontalPosition);

        long min = 999999999;
        int max = horizontalPosition.get(0);
        for(int i : horizontalPosition) {
            if(i > max) max = i;
        }
        for(int i = 0; i <= max; i++) {
            long tmp = costMoveTo2(i);
            if(tmp < min) min = tmp;
        }
        return min + "";
    }

    private long costMoveTo2(int pos) {
        long cpt = 0;
        for(int i : horizontalPosition) {
            if(i < pos) {
                int t = 0;
                for(int j = 0; j <= pos - i; j++) t += j;
                cpt += t;
            }
            if(i > pos) {
                int t = 0;
                for(int j = 0; j <= i - pos; j++) t += j;
                cpt += t;
            }
        }
        return cpt;
    }
}
