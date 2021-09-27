import sys

n = int(sys.argv[1])
seq_name = "@A00133:401:HGHCHDSX2:4:1101:5385:2143 1:N:0:TCCTGAGC+ATAGAGAG"
seq = "ACCTCCAACGAACATGTAGGTCTCAACAACCAGACTACGTATTAGCATACGCTCGACAGTCGACGCTGGTTTTTTTTTTTTTTTTTTTTAAAAAATGGTGGTTTATATTTTTTTTAAAAATTATTACAAAGCCAAACCAATTAAATGCCC"
qual = "F" * len(seq)

n_million = n // 1000000
with open(f"testP{n_million}M_1.fq", 'w') as writer:
    for _ in range(n):
        writer.write(f"{seq_name}\n{seq}\n+\n{qual}\n")