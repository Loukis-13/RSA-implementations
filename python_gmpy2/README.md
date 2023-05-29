# Python RSA  
Algoritmo de criptografia RSA implementado em Python.  

## Como usar  
**Digita `./RSA -h` para obter ajuda sobre os subcomandos que podem ser utilizados.**  

### Keygen  
Automaticamente gera os pares de chaves publicas e privadas e as salva em arquivos separados
```bash
# Para gerar as chaves a partir de valores aleatórios
./RSA keygen

# Para definir os valores com quais as chaves serão geradas e imprimi-las
./RSA keygen -p 7 -q 11 --print

# Para mais ver mais opções
./RSA keygen --help
```

### Encript  
Encrípta um texto utilizando as chaves públicas. **O tamanho máximo do texto que poderá ser encriptado dependerá da quantidade de bits das chaves que foram geradas.**  
```bash
# Para encriptar um texto usando o par de chaves públicas
./RSA encrypt 'FATEC' 5B8EBDC017 6D4DA1BDED

# Ou para usar as chaves direto do arquivo gerado
./RSA encrypt 'FATEC' $(cat RSA-key.pub)

# Para obter mais ajuda
./RSA encrypt --help
```

### Decrypt  
Desencrípta um texto criptografado utilizando as chaves privadas.
```bash
# Para desencriptar um texto usando o par de chaves privadas
./RSA decrypt '736A0C846' E43485143 6D4DA1BDED 

# Ou para usar as chaves direto do arquivo gerado
./RSA decrypt '736A0C846' $(cat RSA-key)

# Para obter mais ajuda
./RSA decrypt --help
```

