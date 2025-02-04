Salam! Builder dizayn patterini (qurucu dizayn nümunəsi) izah edim.  

## **Builder Dizayn Patterni Nədir?**  
Builder patterni, obyektlərin kompleks yaradılmasını asanlaşdırmaq üçün istifadə edilən bir dizayn nümunəsidir. Əsas məqsədi, bir obyektin yaradılma prosesini onun təmsil edilməsindən ayırmaqdır. Bu pattern xüsusilə, çox sayda parametr tələb edən obyektlərin tərtibi zamanı faydalıdır.  

## **Nə Üçün Lazımdır?**  
Əgər bir sinifin konstrukturu çox sayda parametr qəbul edirsə və bu parametrləri fərqli kombinasiyalarda istifadə etmək lazımdırsa, Builder patternindən istifadə etmək daha oxunaqlı və çevik bir yanaşma təmin edir.  

## **İşləmə Prinsipi:**  
1. **Builder (Qurucu)** – Obyektin fərdi hissələrini yaratmaq üçün metodları təyin edir.  
2. **ConcreteBuilder (Konkret Qurucu)** – Obyektin yaradılma qaydasını təyin edən sinifdir.  
3. **Director (Direktor)** – Obyektin yaradılma ardıcıllığını idarə edir.  
4. **Product (Məhsul)** – Yaradılan obyektin özüdür.  

### **İzah:**  
- `CarBuilder` sinfi **builder** kimi işləyir və `Car` obyektini qurur.  
- `year` metodundan istifadə etməsək, default dəyər `2024` olacaq.  
- `build` metodu **Car** obyektini qaytarır.  

Bu patterni istifadə edərək, kodun oxunaqlılığını və çevikliyini artırmaq mümkündür.  
