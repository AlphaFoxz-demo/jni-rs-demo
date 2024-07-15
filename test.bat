cargo build --release
cd java
del *.class
javac ./Test.java
java Test