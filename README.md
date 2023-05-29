# RSA Implementations
Implementations of the [RSA](https://en.wikipedia.org/wiki/RSA_(cryptosystem)) algorithm in different languages and libraries

## Performance
<table>
<thead>
  <tr>
    <th rowspan="2">Language and Lib</th>
    <th colspan="4">Time taken to generate a key of n bits</th>
  </tr>
  <tr>
    <th>1024</th>
    <th>2048</th>
    <th>4096</th>
    <th>10000</th>
  </tr>
</thead>
<tbody>
  <tr>
    <td>Python gmpy2</td>
    <td>0.056s</td>
    <td>0.059s</td>
    <td>0.059s</td>
    <td>1m 50s</td>
  </tr>
  <tr>
    <td>Rust Rug</td>
    <td>0.127s</td>
    <td>1s</td>
    <td>13s</td>
    <td>4m 30s</td>
  </tr>
  <tr>
    <td>Rust Bignum</td>
    <td>0.145s</td>
    <td>1.7s</td>
    <td>14s</td>
    <td>+20m</td>
  </tr>
  <tr>
    <td>Python</td>
    <td>1s</td>
    <td>20s</td>
    <td>2m 30s</td>
    <td>+20m</td>
  </tr>
</tbody>
</table>

### *Specifications of the system in which the data was collected
OS: Manjaro Linux x86_64  
CPU: AMD Ryzen 5 3500U with Radeon Vega Mobile Gfx (8) @ 2.100GHz  
GPU: AMD ATI Radeon Vega Series / Radeon Vega Mobile Series  
Memory: 17865MiB  
