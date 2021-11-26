#!/usr/bin/env python

def aufbau(electronNum, element="X"):
    # Aufbau rule = 1s^2 2s^2 2p^6 3s^2 3p^6 4s^2 3d^10 4p^6 5s^2 4d^10 5p^6 6s^2 4f^14 5d^10 6p^2 7s^2 5f^14 6d^10 7p^6
    # max num can be 118 (118th element is Ogassenon) 
    mlDict = {"s" : 0, "p" : 1, "d" : 2, "f" : 3} 
    orbitals = ["1s^", "2s^", "2p^", "3s^", "3p^", "4s^", "3d^", "4p^", "5s^", "4d^", "5p^", "6s^", "4f^", "5d^", "6p^", "7s^", "5f^", "6d^", "7p^"] 
    exceptions = {24 : "1s^2 2s^2 2p^6 3s^2 3p^6 4s^1 3d^5" , 29 : "1s^2 2s^2 2p^6 3s^2 3p^6 4s^1 3d^10"}

    # Variables
    configuration = ""
    configurationStep = 0
    e = 0

    # Result text
    result = "Configuration of Element " + str(element) + " is: "

    
    if (electronNum == 29) or (electronNum == 24):
        return(result + exceptions[electronNum])
    else:    
        while True:
            l = orbitals[configurationStep][1]
            ml = mlDict[l]
            mln = (2*ml+1)*2
            if (electronNum > mln):
                e = mln
                orbital = orbitals[configurationStep]
                configuration += orbital + str(e) + " "
                configurationStep += 1
                electronNum -= mln
            elif (electronNum == mln):
                e = mln
                orbital = orbitals[configurationStep]
                configuration += orbital + str(e) + " "
                return(result + configuration)
                break
            elif (electronNum < mln):
                e = electronNum
                orbital = orbitals[configurationStep]
                configuration += orbital + str(e) + " "
                return(result + configuration)
                break

while True:
    try:
        eNum = int(input("Type the number: "))
        print(aufbau(eNum,"that you selected"))
        break
    except ValueError:
        print("You typed wrong. Please enter a number")

