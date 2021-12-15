package com.aoc.Day04;

import com.aoc.Day;

import java.util.*;

public class Day04 implements Day {
    @Override
    public String part1(List<String> input) {
        List<String> numberDrawnString = new ArrayList<>(Arrays.asList(input.get(0).split(",")));
        Map<Integer, List<List<String>>> boards = parser(input);
        for(String i : numberDrawnString) {
            for(int j : boards.keySet()) {
                if(checkForValue(boards.get(j), i)) {
                    return result(boards.get(j), i);
                }
            }
        }
        return null;
    }

    public Map<Integer, List<List<String>>> parser(List<String> input) {
        Map<Integer, List<List<String>>> boards = new HashMap<>();
        List<List<String>> board = new ArrayList<>();

        for(int i = 2; i < input.size(); i++){
            if(input.get(i).equals("")){
                boards.put(boards.size(), board);
                board = new ArrayList<>();
            } else {
                List<String> row = new ArrayList<>();
                for(String j : input.get(i).split(" {2}")) {
                    row.addAll(Arrays.asList(j.split(" ")));
                }
                board.add(row);
            }
        }
        boards.put(boards.size(), board);
        return boards;
    }

    public boolean checkForValue(List<List<String>> board, String value) {
        for(List<String> i : board) {
            for(String j : i) {
                if(j.equals(value)) {
                    board.get(board.indexOf(i)).set(i.indexOf(j), "-1");
                    if (checkWin(board)) return true;
                }
            }
        }
        return false;
    }

    public boolean checkWin(List<List<String>> board) {
        boolean vic = true;
        for(List<String> i : board) {
            for(String j : i) {
                if(!j.equals("-1")) {
                    vic = false;
                    break;
                }
            }
            if(vic) return true;
        }
        for(int i = 0; i < 5; i++) {
            if(board.get(i).get(0).equals("-1") && board.get(i).get(1).equals("-1") && board.get(i).get(2).equals("-1")
                && board.get(i).get(3).equals("-1") && board.get(i).get(4).equals("-1")) return true;
        }
        return false;
    }

    public String result(List<List<String>> board, String value) {
        int res = 0;
        for(List<String> i : board) {
            for(String j : i) {
                if(!j.equals("-1")) res += Integer.parseInt(j);
            }
        }
        return res * Integer.parseInt(value) + "";
    }

    @Override
    public String part2(List<String> input) {
        return null;
    }
}
