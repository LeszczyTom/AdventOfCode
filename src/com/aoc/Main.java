package com.aoc;

import com.aoc.Day01.Day01;
import com.aoc.Day02.Day02;
import com.aoc.Day03.Day03;
import com.aoc.Day04.Day04;
import com.aoc.Day05.Day05;
import com.aoc.Day06.Day06;
import com.aoc.Day07.Day07;
import com.aoc.Day08.Day08;

import java.io.*;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Objects;

import static java.util.stream.Collectors.toList;

class App {

    private static final Map<Integer, Day> DAYS;

    static {
        DAYS = new HashMap<>();
        DAYS.put(1, new Day01());
        DAYS.put(2, new Day02());
        DAYS.put(3, new Day03());
        DAYS.put(4, new Day04());
        DAYS.put(5, new Day05());
        DAYS.put(6, new Day06());
        DAYS.put(7, new Day07());
        DAYS.put(8, new Day08());
    }

    private static List<String> loadInput(int day){
        String paddedDay = String.valueOf(day);
        if(day < 10) {
            paddedDay = "0" + day;
        }
        String fileName = "src/ressources/day" + paddedDay + ".txt";

        try(BufferedReader r = new BufferedReader(new FileReader(fileName))){
            return r.lines().collect(toList());
        } catch(IOException e){
            throw new UncheckedIOException(e);
        }
    }

    public static void main(String[] args) {
        int day = 1;
        if(args.length != 0){
            day = Integer.parseInt(args[0]);
        }

        int part = 1;
        if(args.length > 1){
            part = Integer.parseInt(args[1]);
        }

        List<String> input = loadInput(day);

        String result;
        if(part == 1) {
            result = DAYS.get(day).part1(input);
        } else {
            result = DAYS.get(day).part2(input);
        }

        System.out.println(result);
    }
}