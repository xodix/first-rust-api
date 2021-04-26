use maturainf;
SELECT `id_asortymentu`, SUM(`ilosc`) as `ilosc_calkowita` FROM `transakcje` GROUP BY `id_asortymentu` ORDER BY `ilosc_calkowita` DESC;
SELECT `id_asortymentu`, SUM(`ilosc`) as `ilosc_calkowita` FROM `transakcje` GROUP BY `id_asortymentu` ORDER BY `ilosc_calkowita` DESC HAVING `ilosc_calkowita`>10000;