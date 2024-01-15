#![allow(warnings)]

use rand::Rng;


fn main() {
    
    //Referencias e ideas desechadas, ultima revision V.26
    //Agregado metodo: numero_a_ecuacion RAPIDO V.28
    //quitando metodos innecesarios V.29

    /* lista de caracteres probados
    1981
       2,147,483,645 ==> probado en calculo de funcion rapido!
    3,924,361
    3493161
    431200
    4312
    4313
    4315
    819668413 ==> resultado con u64: 819688657, diferencia: overflow
    181646532
    85465218
    8 es divisible asimetricamente b es impar
    145 ==> cuadrados simetricos de base 8, PARES
    105 ==> Cuadrados simetricos de base 7, IMPARES

    962212557 rectangulo:ASIMETRICO, lado_cuadrado B: IMPAR, Cuadrado Z: SUPERIOR, Y: NEGATIVA
    
    */

    println!("____________ORIGENES_____________");
    let mut valor_numero: i32 = 962212557; //rand::thread_rng().gen_range(1,999999999); // 18446744073709551615 ==> valor que no rebasa los limites de memoria, maxvalue i32=999999999);
    
    println!("____________FORMA (Z*N)+Y_____________");
    
    //Se realizan 2 operaciones para obtener 2 VALORES COMPUESTOS: (Z*2)+Y:

    let (my_z_1, my_y):(i32,i32) = numero_a_ecuacion(valor_numero);
    let (my_z_2, my_y_2):(i32,i32) = numero_a_ecuacion(7868);//rand::thread_rng().gen_range(1,9999));
    
    
    conversion_ZCuadrada_hacia_ZExponente(256);
    

}

//Llegaron para quedarse:
fn numero_a_ecuacion (my_number: i32) -> (i32,i32) { //METODO FUNCIONAL, ESTA ES LA CLONACION (Para corregir dentro del metodo clonado: RESULTADO "DESFASE Y FINAL" ==> solo VALOR positivo)

    println!("REVISION_numero es: {}",my_number);
    let mut valor_lex : i32 = 0;

    if(my_number<0)// si se proporciona un numero negativo
    {
        valor_lex = (my_number*-1); //Convertimos ese numero a positivo
    } 
    else { //
        valor_lex = my_number; //si es positivo se deja tal cual
    }

    
    let mut valor_referencia: i32 = valor_lex; //sobrante factorial_valor
    let mut counter_referencia: i32 = 0; //factorial_valor
    let mut modificacion_1: i32 = 0;

    if (valor_referencia % 2) == 0 {
        valor_referencia = valor_lex/2;
        modificacion_1 = 0;
        println!("valor_referencia/2 : {}",valor_referencia);
    }
    //numero con decimal al dividir entre 2
    else {
        valor_referencia = (valor_lex+1)/2;
        modificacion_1 = 1;
        println!("valor_referencia/2 : {}",valor_referencia);
    }

    while valor_referencia>=counter_referencia {
        counter_referencia+=1;
        valor_referencia=valor_referencia-counter_referencia;
    }

    println!("ResultadoTRIANGULAR_Factorial:{}",counter_referencia);
    println!("SobranteTRIANGULAR_Factorial:{}",valor_referencia);
    

    //valor_referencia es el residuo del factorial
    //contador_factorial es ahora la base y altura de nuestro triangulo de valor

    //convertimos nuestro triangulo de valor en cuadrado
    //establecemos la altura del cuadrado de valor
    println!("_______________________________");
    let mut base_altura_cuadrado_z: i32 = 0;
    let mut desfase_y: i32 = 0;
    let mut resultado_final: i32 = 0;

    base_altura_cuadrado_z = counter_referencia;
    
    desfase_y = (valor_referencia*2)+(base_altura_cuadrado_z-modificacion_1);
            
        println!("Comprobacion: numero origen: X = (z^2)+(y)");
        println!("Comprobacion: {} = ({}^2)+({})",valor_lex,base_altura_cuadrado_z,desfase_y);
        println!("Comprobacion: {} = ({})+({})",valor_lex,base_altura_cuadrado_z*base_altura_cuadrado_z,desfase_y);
        println!("Comprobacion: {} = ({})",valor_lex,(base_altura_cuadrado_z*base_altura_cuadrado_z) + desfase_y);

        resultado_final= ((base_altura_cuadrado_z*base_altura_cuadrado_z)+desfase_y); 

    println!("___________________________________");
    println!("____________resultados_____________");

    let mut my_return : (i32,i32) = (0,0);

        my_return = (base_altura_cuadrado_z,desfase_y);

    //println!("valor de referencia aka. residuo factorial{}",valor_referencia);

    println!("DESFASE_Y:{}",(my_return.1));
    println!("BASE Z ELEVADA 2:{}",(my_return.0)*(my_return.0));
    
    println!("____________Resumen_____________");

    println!("NUMERO: {}",valor_lex);
    println!("resultado: {}",resultado_final);
    println!("desfase entre valor y resultado:{}",valor_lex-resultado_final);

    println!("________________________________________________");
    println!("______________ECUACION Z^2 + Y__________________");
    println!(" ");
    println!("              (Z^2)+Y = ({})^2+({})",my_return.0,my_return.1);
    println!("________________________________________________");

    return my_return;
}

fn potencias(base: i32,potencia:i32) -> (i32) { //Funcional Y SIN ERRORES

    //Este metodo todo lo que hace es realizar una operacion de potencia nada de otro mundo, de modo que: "BASE ^ POTENCIA = VALOR_RETORNADO"

    let mut my_return_potencia: i32=0;
    let mut var_base:i32=0;
    let mut var_potencia:i32=0;

    if(base>=0){
        var_base=base;
    }
    else {
        var_base=base*-1;
    }

    if(potencia>=0){
        var_potencia=potencia;
    }
    else {
        var_potencia=potencia*-1;
    }

    //println!("MY BASE:{}",var_base);
    //println!("MY POTENCIA:{}",var_potencia);
    

    if(var_potencia>1)
    {
        my_return_potencia = var_base;

        while var_potencia>1 {
            my_return_potencia = my_return_potencia*var_base;
            var_potencia=var_potencia-1;
        }
    }
    else {
        if(var_potencia==0 || var_potencia==1)
        {
            my_return_potencia=var_base;
        }

    }

    if(base>=0){
        my_return_potencia=my_return_potencia;
    }
    else {
        my_return_potencia=my_return_potencia*-1;
    }

    //println!("MY RESULTADO:{}",my_return_potencia);
    return my_return_potencia;
}

//NOTA: 77 es el exponente ideal, pero aparentemente las computadoras actuales solo puede trabajar en RAM con exponentes 20 de manera precisa.

//ZONA DE TRABAJO
//ZONA DE TRABAJO
//ZONA DE TRABAJO
//ZONA DE TRABAJO
//ZONA DE TRABAJO
//ZONA DE TRABAJO
//ZONA DE TRABAJO
//ZONA DE TRABAJO
//ZONA DE TRABAJO
//ZONA DE TRABAJO
//ZONA DE TRABAJO
//ZONA DE TRABAJO


fn conversion_ZCuadrada_hacia_ZExponente(za: i32) { // -> (i32) { //Funcional, pero presenta errores al trabajar con potencias, ya que existen algunos resultados de potencias que no pueden procesarse en memoria y provocan: Error "OVERFLOW"

//Necesito encontrar una forma de trabajar solo con aproximaciones inferiores y descartar las aproximaciones superiores.

    let mut cuadrado_z : i32 = (za*za);

    let mut memoria_cuadrado_z : i32 = cuadrado_z;

    let mut memoria_base_z: i32 = za;
    let mut memoria_exponente_z: i32 = 2;

    let mut exponentes_permitidos: i32 = 20;

    let mut counter_coincidencias: i32 = 3;
    let mut counter_ciclos_realizados: i32 = 0;

    let mut memoria_aproximacion_superior: i32 = 0;
    let mut memoria_aproximacion_inferior: i32 = 0;


    //iniciamos las variables....
    if( (    potencias((memoria_base_z-1),(memoria_exponente_z+1)) > cuadrado_z  ) && cuadrado_z>4 && memoria_base_z>1 && memoria_exponente_z>1) // con ello: verifico que la primera operacion pueda ser realizada, evito bases 1 y mantengo siempre la operacion en exponentes mayores a 1
    {
        memoria_aproximacion_inferior = potencias((memoria_base_z-1),(memoria_exponente_z));
        memoria_aproximacion_superior = potencias((memoria_base_z-1),(memoria_exponente_z+1));

        println!("INICIO_memoria_aproximacion_inferior: z{}^n{} =  {}",memoria_base_z,memoria_exponente_z,memoria_aproximacion_inferior);
        println!("INICIO_memoria_aproximacion_superior: z{}^n{} =  {}",memoria_base_z,memoria_exponente_z+1,memoria_aproximacion_superior);

        memoria_base_z = memoria_base_z-1;

    } 
    else
    {
        memoria_aproximacion_inferior = potencias((memoria_base_z-1),(memoria_exponente_z+1));
        memoria_aproximacion_superior = potencias((memoria_base_z-1),(memoria_exponente_z+2));

        memoria_base_z = memoria_base_z-1;
        memoria_exponente_z = memoria_exponente_z+1;

        println!("INICIO_INF_{}_memoria_aproximacion_inferior: z{}^n{} =  {}",counter_ciclos_realizados,memoria_base_z,memoria_exponente_z,memoria_aproximacion_inferior);
        println!("INICIO_INF_{}_memoria_aproximacion_superior: z{}^n{} =  {}",counter_ciclos_realizados,memoria_base_z,memoria_exponente_z+1,memoria_aproximacion_superior);

    }

    while(counter_coincidencias>0 && memoria_exponente_z>1 && memoria_exponente_z <21 && memoria_base_z>2)
    {
        counter_ciclos_realizados+=1;
        
        
        if(memoria_aproximacion_superior>cuadrado_z){

            memoria_aproximacion_inferior = potencias((memoria_base_z-1),(memoria_exponente_z));
            memoria_aproximacion_superior = potencias((memoria_base_z-1),(memoria_exponente_z+1));

            memoria_base_z = memoria_base_z-1;
    
            //println!("CONTINUA_{}_memoria_aproximacion_inferior: z{}^n{} =  {}",counter_ciclos_realizados,memoria_base_z,memoria_exponente_z,memoria_aproximacion_inferior);
            //println!("CONTINUA_{}_memoria_aproximacion_superior: z{}^n{} =  {}",counter_ciclos_realizados,memoria_base_z,memoria_exponente_z+1,memoria_aproximacion_superior);
    


        }
        else{
            if (  memoria_aproximacion_superior==cuadrado_z || memoria_aproximacion_inferior==cuadrado_z ) {

                if(memoria_aproximacion_superior == cuadrado_z)
                {
                    println!("_______________________________________________________coincidencia encontrada para el valor origen: z{}^n{} =  {}",za,2,cuadrado_z);
                    println!("EXACTO: z{}^n{} =  {} :::: origen: {} ", memoria_base_z, memoria_exponente_z+1, memoria_aproximacion_superior, cuadrado_z);

                    //se realiza el siguiente ciclo:
                    memoria_aproximacion_inferior = potencias((memoria_base_z-1),(memoria_exponente_z));
                    memoria_aproximacion_superior = potencias((memoria_base_z-1),(memoria_exponente_z+1));

                    memoria_base_z = memoria_base_z-1;
            
                    println!("CONTINUA_SUPIGUAL_{}_memoria_aproximacion_inferior: z{}^n{} =  {}",counter_ciclos_realizados,memoria_base_z,memoria_exponente_z,memoria_aproximacion_inferior);
                    println!("CONTINUA_SUPIGUAL_{}_memoria_aproximacion_superior: z{}^n{} =  {}",counter_ciclos_realizados,memoria_base_z,memoria_exponente_z+1,memoria_aproximacion_superior);


                }

                if(memoria_aproximacion_inferior == cuadrado_z)
                {
                    println!("_______________________________________________________coincidencia encontrada para el valor origen: z{}^n{} =  {}",za,2,cuadrado_z);
                    println!("EXACTO: z{}^n{} =  {} :::: origen: {} ", memoria_base_z, memoria_exponente_z, memoria_aproximacion_inferior, cuadrado_z);
    
                    memoria_aproximacion_inferior = potencias((memoria_base_z-1),(memoria_exponente_z));
                    memoria_aproximacion_superior = potencias((memoria_base_z-1),(memoria_exponente_z+1));

                    memoria_base_z = memoria_base_z-1;
            
                    println!("CONTINUA_INFIGUAL_{}_memoria_aproximacion_inferior: z{}^n{} =  {}",counter_ciclos_realizados,memoria_base_z,memoria_exponente_z,memoria_aproximacion_inferior);
                    println!("CONTINUA_INFIGUAL_{}_memoria_aproximacion_superior: z{}^n{} =  {}",counter_ciclos_realizados,memoria_base_z,memoria_exponente_z+1,memoria_aproximacion_inferior);


                } 

            }
            else
            { //(memoria_aproximacion_superior<cuadrado_z)
                memoria_aproximacion_inferior = potencias((memoria_base_z-1),(memoria_exponente_z+1));
                memoria_aproximacion_superior = potencias((memoria_base_z-1),(memoria_exponente_z+2));

                memoria_base_z = memoria_base_z-1;
                memoria_exponente_z = memoria_exponente_z+1;

                println!("CONTINUA_INF_{}_memoria_aproximacion_inferior: z{}^n{} =  {}",counter_ciclos_realizados,memoria_base_z,memoria_exponente_z,memoria_aproximacion_inferior);
                println!("CONTINUA_INF_{}_memoria_aproximacion_superior: z{}^n{} =  {}",counter_ciclos_realizados,memoria_base_z,memoria_exponente_z+1,memoria_aproximacion_superior);


            }


        }
            
    }

    //let my_return4 : (i32) = (zz);

    //return my_return4;

}







//ZONA DE EXCLUSION TEMPORAL
//ZONA DE EXCLUSION TEMPORAL
//ZONA DE EXCLUSION TEMPORAL
//ZONA DE EXCLUSION TEMPORAL
//ZONA DE EXCLUSION TEMPORAL
//ZONA DE EXCLUSION TEMPORAL
//ZONA DE EXCLUSION TEMPORAL
//ZONA DE EXCLUSION TEMPORAL
//ZONA DE EXCLUSION TEMPORAL
//ZONA DE EXCLUSION TEMPORAL
//ZONA DE EXCLUSION TEMPORAL



fn suma_cuadrados (z1:i32,z2:i32) -> (i32,i32) { //METODO FUNCIONAL
    
    println!("REALIZANDO SUMA DE CUADRADOS!!");
  
    let mut z_a: i32 = z1;
    let mut z_b: i32 = z2;

    println!("cuadrado_a:{}",z_a);
    println!("cuadrado_b:{}",z_b);

    //Ambos positivos ó ambos negativos

    //para sumar (+a)+(-b) ==> usar resta_cuadrados (-b)+(a) == (a)-(b)
    //Para sumar (-a)+(b) ==> usar resta_cuadrados (-a)+(b) == (b)-(a)

    let mut z3: i32 = 0;
    let mut y3: i32 = 0;


    if(z_a>0){//z_a = positivo
        //z_a positiva
    }
    else {
        z_a = -1*z_a; //z_a convertida a positiva

        println!("z_a:{} convertida a positiva",z_a);
    }

    if(z2>0){//z_b = positivo
        z_b = z2;
    }
    else {
        z_b = -1*z2;
        println!("z_b:{} convertida a positiva",z_b);
    }

    if(z_a>z_b) {

        z_a=z_a;
        z_b=z_b;

        println!("za>zb");
    }
    else { //z1==z2 || z2<zb
        
        if(z1==z2){
            z_a= z_a;
            z_b= z_a;
        }
        else {
            let temporal_z_a:i32 = z_a;
            z_a = z_b;
            z_b = temporal_z_a;
        }
    }

    //Verificar si son consecutivos
    //para cuadrados inferiores
    let mut base_cuadrito_n_2: i32 = 0; //n
    let mut base_cuadrado_z_2: i32 = 0; //z
    let mut base_cuadrito_m_2: i32 = 0; //m
    let mut residuo_y_2: i32 = 0; //y
    let mut residuo_temporal_b_2:i32 = 0;

    
    
    if(z_a == z_b) { //Son iguales
        println!("A=B!!!!!!!!");
        //USAR EL METODO SIMETRICO INICIO

        if (z_b % 2) == 0{ //b ES PAR SIMETRICO
            println!("______________B ES PAR_________________");
    
            base_cuadrito_n_2 = (z_b / 2)-1;
            base_cuadrado_z_2 = z_a + z_b;
            residuo_y_2 = (z_b*2)-(base_cuadrito_n_2 * base_cuadrito_n_2); //agregado base del cuadrado b
                
            println!("Comprobacion: (a^2)+(b^2)=(z^2)+(y)");
            println!("Comprobacion: ({}^2)+({}^2)=({}^2)+({})",z_a,z_b,base_cuadrado_z_2,residuo_y_2);
            println!("Comprobacion: ({})+({})=({})+({})",z_a*z_a,z_b*z_b,base_cuadrado_z_2*base_cuadrado_z_2,residuo_y_2);
            println!("Comprobacion: ({})=({})",(z_a*z_a)+(z_b*z_b),(base_cuadrado_z_2*base_cuadrado_z_2)+residuo_y_2);
            println!("Comprobacion: residuo y({})",residuo_y_2);
    
            let mut resultado_final_: i32= ((base_cuadrado_z_2*base_cuadrado_z_2)+residuo_y_2);

            z3= base_cuadrado_z_2;
            y3= residuo_y_2; 
    
        }
        else{//b ES IMPAR SIMETRICO
            println!("______________B ES IM-PAR_________________");
    
            base_cuadrito_n_2 = (z_b -1)/2;
            base_cuadrado_z_2 = z_a + base_cuadrito_n_2;
            residuo_y_2 = (z_b)-(base_cuadrito_n_2 * base_cuadrito_n_2); //agregado base del cuadrado b
                
            println!("Comprobacion: (a^2)+(b^2)=(z^2)+(y)");
            println!("Comprobacion: ({}^2)+({}^2)=({}^2)+({})",z_a,z_b,base_cuadrado_z_2,residuo_y_2);
            println!("Comprobacion: ({})+({})=({})+({})",z_a*z_a,z_b*z_b,base_cuadrado_z_2*base_cuadrado_z_2,residuo_y_2);
            println!("Comprobacion: ({})=({})",(z_a*z_a)+(z_b*z_b),(base_cuadrado_z_2*base_cuadrado_z_2)+residuo_y_2);
            println!("Comprobacion: residuo y({})",residuo_y_2);
    

            let mut resultado_final_ :i32 = ((base_cuadrado_z_2*base_cuadrado_z_2)+residuo_y_2);
                
            z3= base_cuadrado_z_2;
            y3= residuo_y_2;   
        }
        
        //USAR EL METODO SIMETRICO FIN
    }
    else {
        if (z_a-1 == z_b && z_b+1 == z_a) { //Son consecutivos
            


            println!("A>B;B+1=A;A-1=B SON CONSECUTIVOS!!!!!!!!");
            //USAR EL METODO ASIMETRICO INICIO

            if(z_b % 2) == 0 { 

                println!("______________B ES PAR_________________"); //PARA CUANDO B ES PAR ASIMETRICO:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
    
                base_cuadrito_n_2 = ((z_b/2)-1); 
                base_cuadrado_z_2=z_a+base_cuadrito_n_2;
                residuo_temporal_b_2 = (z_b*2)-(base_cuadrito_n_2*2); 
    
                residuo_y_2 = ((residuo_temporal_b_2)-(base_cuadrito_n_2*base_cuadrito_n_2));
    
    
                println!("Comprobacion: (a^2)+(b^2)=(z^2)+(y)");
                println!("Comprobacion: ({}^2)+({}^2)=({}^2)+({})",z_a,z_b,base_cuadrado_z_2,residuo_y_2);
                println!("Comprobacion: ({})+({})=({})+({})",z_a*z_a,z_b*z_b,base_cuadrado_z_2*base_cuadrado_z_2,residuo_y_2);
                println!("Comprobacion: ({})=({})",(z_a*z_a)+(z_b*z_b),(base_cuadrado_z_2*base_cuadrado_z_2)+residuo_y_2);
                println!("Comprobacion: residuo y({})",residuo_y_2);
    

                let mut resultado_final_ :i32 = ((base_cuadrado_z_2*base_cuadrado_z_2)+residuo_y_2);
                
                z3= base_cuadrado_z_2;
                y3= residuo_y_2;
    
                
            }
            else { //para cuando B ES IMPAR
    
                println!("______________B ES IM-PAR_________________"); //PARA CUANDO B ES IMPAR ASIMETRICO:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
    
                base_cuadrito_n_2=(z_b-1)/2;
                base_cuadrado_z_2=z_a + base_cuadrito_n_2;
                residuo_temporal_b_2=(z_b)-(base_cuadrito_n_2*2);
                residuo_y_2 = (residuo_temporal_b_2-(base_cuadrito_n_2*base_cuadrito_n_2));
                
    
                println!("Comprobacion: (a^2)+(b^2)=(z^2)+(y)");
                println!("Comprobacion: ({}^2)+({}^2)=({}^2)+({})",z_a,z_b,base_cuadrado_z_2,residuo_y_2);
                println!("Comprobacion: ({})+({})=({})+({})",z_a*z_a,z_b*z_b,base_cuadrado_z_2*base_cuadrado_z_2,residuo_y_2);
                println!("Comprobacion: ({})=({})",(z_a*z_a)+(z_b*z_b),(base_cuadrado_z_2*base_cuadrado_z_2)+residuo_y_2);
                println!("Comprobacion: residuo y({})",residuo_y_2);
    

                let mut resultado_final_ :i32 = ((base_cuadrado_z_2*base_cuadrado_z_2)+residuo_y_2);
                
                z3= base_cuadrado_z_2;
                y3= residuo_y_2;
                
    
            }

            //USAR EL METODO ASIMETRICO FIN

        }
        else{ // No son consecutivos
            
            print!("NO SON CONSECUTIVOS!!!!!!!");

            if(z_b%2==0){ //numero par

                println!("Entro numero PAR en b");
                base_cuadrito_m_2 = (z_b/2);
                base_cuadrado_z_2 = z_a+base_cuadrito_m_2;
                base_cuadrito_n_2 = z_a-z_b;
                residuo_y_2 = -1*((base_cuadrito_m_2*base_cuadrito_m_2)+((base_cuadrito_m_2*base_cuadrito_n_2)*2));

                z3= base_cuadrado_z_2;
                y3= residuo_y_2;

            }
            else {
                println!("entro numero IMPAR en b");
                base_cuadrito_m_2 = (z_b-1)/2;
                base_cuadrado_z_2 = z_a+base_cuadrito_m_2;
                base_cuadrito_n_2 = z_a-z_b;
                residuo_y_2 = ((-1)*((base_cuadrito_m_2*base_cuadrito_m_2)+((base_cuadrito_m_2*base_cuadrito_n_2)*2))) + z_b;

                z3= base_cuadrado_z_2;
                y3= residuo_y_2;
                
            }

        }
    }

    
    let mut my_return3 : (i32,i32) = (0,0);

    if z1>=0 && z2>=0  { //positivo
        my_return3 = (z3,y3) ;
    }
    else{
        my_return3 = (-1*z3,-1*y3) ;
    }

    println!("Z final suma:{}",my_return3.0);
    println!("y final suma:{}",my_return3.1);

    return my_return3;

}

