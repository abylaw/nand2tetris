// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// Put your code here.

(MAIN)
	@8192 // the number of 16-bit words in the screen address space.
	D=A
	@i
	M=D		// now it's possible to iterate backwards through @i to get each row,
	@KBD	// starting at the last one. The (DRAW) loop does this.
	D=M
	@CLEAR
	D;JEQ
	@n
	M=-1 	// @n = -1 to make a whole row black
	(DRAW)
		@i
		D=M
		@MAIN
		D;JLT
		@SCREEN
		D=D+A
		@addr
		M=D		//set @addr to the address of the row to be worked on.
		@n
		D=M
		@addr
		A=M
		M=D		//set the word at address @addr to the value of @n
		@i
		M=M-1
		@DRAW
		0;JMP
	(CLEAR)
		@n
		M=0	// @n = 0 to make a whole row white
		@DRAW
		0;JMP