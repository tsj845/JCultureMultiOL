#find . -name "*.class" -type f -delete && 
#javac JCRoot/game/Board.java && java JCRoot/game/Board "$@"
javac JCRoot/game/Board.java
java JCRoot/game/Board "-1"
java JCRoot/game/Board "0"
java JCRoot/game/Board "1"
java JCRoot/game/Board "2"
java JCRoot/game/Board "3"
java JCRoot/game/Board "4"
java JCRoot/game/Board "5"