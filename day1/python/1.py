def manufacture_gifts(gifts_to_produce):

    return [gift for gift in gifts_to_produce if "#" not in gift]


gifts1 = ["car", "doll#arm", "ball", "#train"]
good1 = manufacture_gifts(gifts1)
print(good1)
