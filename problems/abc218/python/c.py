def rot(S):
	return list(zip(*S[::-1]))

N = int(input())
S = [input() for _ in range(N)]
T = [input() for _ in range(N)]

print(S[::-1])
print(rot(S))