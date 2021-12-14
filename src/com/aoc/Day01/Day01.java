package com.aoc.Day01;

import com.aoc.Day;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Day01 implements Day {

    @Override
    public String part1(List<String> input) {
        List<Integer> integerList = new ArrayList<>();
        for(String j : input) {
            integerList.add(Integer.parseInt(j));
        }

        int cpt = 0, before = 0;
        for(int j : integerList) {
            if(j > before && before != 0) {
                cpt++;
            }
            before = j;
        }
        return input.isEmpty() ? "" : cpt + "";
    }

    @Override
    public String part2(List<String> input) {
        List<Integer> integerList = new ArrayList<>();
        for(String j : input) {
            integerList.add(Integer.parseInt(j));
        }

        int length = integerList.size(), before = 0, cpt = 0;
        for(int i = 0; i < length - 2; i++) {
            int tmp = integerList.get(i) + integerList.get(i + 1) + integerList.get(i + 2);
            if(tmp > before && before != 0) {
                cpt ++;
            }
            before = tmp;
        }
        return input.isEmpty() ? "" : cpt + "";
    }

}
