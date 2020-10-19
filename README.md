# rest_weather
 weather rest web service


### Задача

RESTful веб-сервис. Сервис должен возвращать прогноз погоды (температуру) в заданном городе:
- на заданный день (текущий или следующие, с историческими данными работать не нужно)
- на ближайшую неделю (коллекция из 5 дней)
В качестве источника данных нужно выбрать пару сторонних веб-сервисов (с открытым API). Нужно вычислить среднее значение по данным из обоих.

В реализации при выборе тех или иных решений, стоит ориентироваться на те, что вы предпочли бы использовать в реальном приложении.

### Выбраны 3 сторонних сервиса
    AccuWeather ( У меня сервис долго отвечает((( )
    OpenWeatherMap
    WeatherBit
    
Каждый из сервисов имеет ограничения на количество запросов в день.

Максимальный прогноз возможен на день, который принадлежит периоду от сегодняшнего дня
> +16 дней ( OpenWeatherMap )
> +16 дней ( WeatherBit )
> +5 дней ( AccuWeather )

По умолчанию возвращается температура в фаренгейт

Чтобы полчить температуру, к url добавить /celsius
    
В конфигурационном файле при необходиимости отредактируйте ключи.

Сейчас ключи рабочие

### Разработка проводилась на Rust 1.47.0

### Примеры запросов и ответов


http://localhost:8080/Ufa/daily/2020-10-19

{"success":true,"error":null,"forecasts":[{"date":"2020-10-19","forecast":{"min":46.3,"max":55.4}}]}

http://localhost:8080/Ufa/daily/2020-10-26

{"success":true,"error":null,"forecasts":[{"date":"2020-10-26","forecast":{"min":38.3,"max":44.9}}]}

http://localhost:8080/Ufa/daily/2020-11-03

{"success":true,"error":null,"forecasts":[{"date":"2020-11-03","forecast":{"min":29.8,"max":32.5}}]}

http://localhost:8080/Ufa/daily/2020-11-04

{"success":false,"error":"Forecast result is unknown. Target day is after MAX_DATE ( MAX_DATE for AccuWeather is 5 days, MAX_DATE for OpenWeatherMap is 16 days, MAX_DATE for WeatherBit is 16 days ) day. Check your Target day.","forecasts":[]}

http://localhost:8080/Ufa/daily/2020-11-05

{"success":false,"error":"Forecast result is unknown. Target day is after MAX_DATE ( MAX_DATE for AccuWeather is 5 days, MAX_DATE for OpenWeatherMap is 16 days, MAX_DATE for WeatherBit is 16 days ) day. Check your Target day.","forecasts":[]}

http://localhost:8080/Ufa/weekly

### Сегодня 19.10.2020

### without AccuWeather

### Fahrenheit

{"success":true,"error":null,"forecasts":[{"date":"2020-10-19","forecast":{"min":46.3,"max":55.4}},{"date":"2020-10-20","forecast":{"min":39.8,"max":46.9}},{"date":"2020-10-21","forecast":{"min":34.0,"max":40.1}},{"date":"2020-10-22","forecast":{"min":30.2,"max":33.7}},{"date":"2020-10-23","forecast":{"min":28.2,"max":34.1}}]}

### with AccuWeather

### Fahrenheit

{"success":true,"error":null,"forecasts":[{"date":"2020-10-19","forecast":{"min":40.3,"max":57.6}},{"date":"2020-10-20","forecast":{"min":36.0,"max":45.9}},{"date":"2020-10-21","forecast":{"min":31.5,"max":41.2}},{"date":"2020-10-22","forecast":{"min":28.1,"max":34.3}},{"date":"2020-10-23","forecast":{"min":24.9,"max":34.4}}]}

### with Celsius

{"success":true,"error":null,"forecasts":[{"date":"2020-10-19","forecast":{"min":5.7,"max":13.7}},{"date":"2020-10-20","forecast":{"min":2.9,"max":8.1}},{"date":"2020-10-21","forecast":{"min":0.2,"max":5.1}},{"date":"2020-10-22","forecast":{"min":-1.8,"max":1.4}},{"date":"2020-10-23","forecast":{"min":-3.2,"max":1.4}}]}