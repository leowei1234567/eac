package main

import (
	"bufio"
	"crypto/ecdsa"
	"crypto/elliptic"
	"crypto/rand"
	"encoding/hex"
	"flag"
	"fmt"
	"log"
	"os"
	"runtime"
	"sync/atomic"
	"time"

	"github.com/ethereum/go-ethereum/crypto"
)

var addressesSet map[string]bool

// 加载地址到集合中
func loadAddresses(filename string) (map[string]bool, error) {
	file, err := os.Open(filename)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	addresses := make(map[string]bool)
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		addresses[scanner.Text()] = true
	}

	return addresses, scanner.Err()
}

// 检查地址是否在集合中
func isAddressInSet(address string) bool {
	_, exists := addressesSet[address]
	return exists
}

// 生成以太坊地址
func generateEthereumAddress() (string, string, string, error) {
	privateKey, err := ecdsa.GenerateKey(crypto.S256(), rand.Reader)
	if err != nil {
		return "", "", "", err
	}
	publicKeyBytes := elliptic.Marshal(crypto.S256(), privateKey.PublicKey.X, privateKey.PublicKey.Y)
	publicKeyHex := hex.EncodeToString(publicKeyBytes)
	address := crypto.PubkeyToAddress(privateKey.PublicKey).Hex()
	privateKeyHex := fmt.Sprintf("%x", privateKey.D)

	return privateKeyHex, address, publicKeyHex, nil

}

func buyLottery() {
	privateKey, address, publicKey, err := generateEthereumAddress()
	if err != nil {
		panic(err)
	}

	if isAddressInSet(address) {
		fmt.Println("Found matching address:", address)
		fmt.Println("Private Key:", privateKey)
		fmt.Println("Publick Key:", publicKey)
		// 打开文件，如果文件不存在则创建，如果存在则在文件末尾追加内容
		file, err := os.OpenFile("lottery.txt", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
		if err != nil {
			log.Fatal(err)
		}
		defer file.Close()
		// 写入信息到文件
		_, err = file.WriteString(fmt.Sprintf("Found matching address: %s\nPrivate Key: %s\nPublic Key: %s\n", address, privateKey, publicKey))
		if err != nil {
			log.Fatal(err)
		}
	}
}

func main() {
	// 定义一个整型命令行参数t，用于指定goroutine的数量
	threadCount := flag.Int("t", runtime.NumCPU(), "number of threads to use")
	flag.Parse() // 解析命令行参数

	fmt.Printf("Lottery Threads  = %d\n", *threadCount)

	var err error
	addressesSet, err = loadAddresses("address.txt")
	if err != nil {
		panic(err)
	}

	var count int64 // 使用 int64 以支持原子操作
	var totalCount int64 = 0

	for i := 0; i < *threadCount; i++ {
		go func() {
			for {
				buyLottery()
				atomic.AddInt64(&count, 1) // 原子地增加计数器
			}
		}()
	}

	go func() {
		for {
			time.Sleep(1 * time.Second)
			cnt := atomic.SwapInt64(&count, 0) // 获取当前计数器的值，并重置为0
			totalCount += cnt
			fmt.Printf("%s: Speed %d lotterys per second, total: %d\n", time.Now().Format("2006-01-02 15:04:05"), cnt, totalCount)
		}
	}()

	select {}

}
