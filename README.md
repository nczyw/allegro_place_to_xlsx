# 自动将Allegro生成的坐标文件转换成xlsl,嘉立创可直接识别
## 使用方法,输出文件后缀名可以不用填
 - `allegro_place_to_xlsx --source place_txt.txt` , 输出`place_txt.xlsx`
 - `allegro_place_to_xlsx` , 输出`place_txt.xlsx` , 因为默认输入文件名字为`place_txt.txt`
 - `allegro_place_to_xlsx --output a` , 输出`a.xlsx` , 因为默认输入文件名字为`place_txt.txt`
 - `allegro_place_to_xlsx --source place_txt.txt --output b.xlsx`,输出`b.xlsx`