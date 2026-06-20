# 📐 Birim Dönüştürücü

Uzunluk, ağırlık, sıcaklık, hacim, veri boyutu ve hız için çok
kategorili birim dönüştürücü. Tek dosyalık, kurulum gerektirmeyen
tarayıcı aracı.

## Özellikler

- 📏 **Uzunluk**: mm, cm, m, km, inç, fit, yarda, mil
- ⚖️ **Ağırlık**: mg, g, kg, ton, ons, pound
- 🌡️ **Sıcaklık**: Celsius, Fahrenheit, Kelvin (doğrusal olmayan dönüşüm dahil)
- 🧪 **Hacim**: ml, litre, galon, su bardağı, yemek/tatlı kaşığı
- 💾 **Veri Boyutu**: bit, byte, KB, MB, GB, TB
- 🚗 **Hız**: m/s, km/sa, mph, knot
- ⇄ Tek tıkla birim değiştirme (swap)
- 📋 "Hızlı referans" — girilen değerin tüm diğer birimlerdeki karşılığını anında gösterme
- 🔬 Çok büyük/küçük sayılar için otomatik bilimsel gösterim

## Kullanım

Kurulum gerekmez:

```bash
open index.html        # macOS
xdg-open index.html     # Linux
```

1. Üstteki sekmelerden bir kategori seçin (Uzunluk, Ağırlık, vb.)
2. Değeri girin, kaynak ve hedef birimi seçin
3. Sonuç anında hesaplanır
4. Alt kısımda, girdiğiniz değerin **tüm diğer birimlerdeki** karşılığını
   tek bakışta görebilirsiniz

## Neden bu araç?

Çoğu online birim dönüştürücü reklam dolu ve yavaş yüklenir. Bu araç
tek bir HTML dosyası — internet bağlantısı olmadan da çalışır,
hiçbir veri göndermez, anında açılır.

## Teknik notlar

- Doğrusal birimler (uzunluk, ağırlık, hacim, veri, hız) bir **baz
  birime göre çarpan** ile tanımlanır — örneğin 1 km = 1000 m, baz
  birim metre olduğundan km'nin çarpanı 1000'dir
- **Sıcaklık özel olarak ele alınır** çünkü dönüşüm doğrusal değildir
  (ölçek farkının yanında bir kayma/offset de vardır) — örneğin
  0°C = 32°F, ama 0°C × 2 ≠ 32°F × 2

## Lisans

MIT


---

> Made in [discord.gg/codeshare](https://discord.gg/codeshare) · [astra-dev.com.tr](https://astra-dev.com.tr)
