package com.aoc.Day06;

import com.aoc.Day;

import java.util.List;

public class Day06 implements Day {
    int cpt = 0;
    @Override
    public String part1(List<String> input) {
        String[] initialState = input.get(0).split(",");
        int days = 80;
        for(String n : initialState) {
            fish(Integer.parseInt(n), days);
        }
        return cpt + "";
    }

    private void fish(int nb, int days) {
        cpt++;
        for(int i = 0; i <= days; i++) {
            nb--;
            if(nb == -1 && days - 1 - i >= 0) {
                nb = 6;
                fish(8, (days - i - 1));
            }
        }
    }

    @Override
    public String part2(List<String> input) {
        return null;
    }
}
