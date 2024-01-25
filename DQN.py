import numpy as np
import torch
import torch.nn as nn
import torch.optim as optim
import time

# Define the 0-1 Knapsack Problem environment
class KnapsackEnvironment:
    def __init__(self, items, capacity):
        self.initial_capacity = capacity
        self.items = items
        self.capacity = capacity
        self.num_items = len(items)
        self.current_item = 0
        self.max_reward = 0

    # Function to get the current state
    def get_state(self):
        return self.current_item, self.capacity

    # Function to take an action and return the next state, reward, and whether the episode is done
    def take_action(self, action):
        item = self.items[self.current_item]
        if action == 1 and item[1] <= self.capacity:
            self.capacity -= item[1]
            reward = item[0]
        else:
            reward = 0
        self.current_item += 1
        done = self.current_item >= self.num_items
        if self.current_item >= self.num_items:
            self.current_item = 0
        return self.get_state(), reward, done

    def reset(self):
        self.current_item = 0
        self.capacity = self.initial_capacity

# Define the Deep Q-Network (DQN) model
class DQN(nn.Module):
    def __init__(self, input_dim, output_dim, hidden_dim=64):
        super(DQN, self).__init__()
        self.fc1 = nn.Linear(input_dim, hidden_dim)
        self.fc2 = nn.Linear(hidden_dim, hidden_dim)
        self.fc3 = nn.Linear(hidden_dim, output_dim)
        
    def forward(self, x):
        x = torch.relu(self.fc1(x))
        x = torch.relu(self.fc2(x))
        x = self.fc3(x)
        return x

# Define the DQN agent
class DQNAgent:
    def __init__(self, input_dim, output_dim, hidden_dim=64, learning_rate=0.001, gamma=0.99, epsilon=1.0, epsilon_min=0.01, epsilon_decay=0.995):
        self.input_dim = input_dim
        self.output_dim = output_dim
        self.epsilon = epsilon
        self.epsilon_min = epsilon_min
        self.epsilon_decay = epsilon_decay
        self.gamma = gamma
        
        self.model = DQN(input_dim, output_dim, hidden_dim)
        self.optimizer = optim.Adam(self.model.parameters(), lr=learning_rate)
        self.criterion = nn.MSELoss()
        
    def get_action(self, state):
        if np.random.rand() <= self.epsilon:
            return np.random.randint(self.output_dim)
        else:
            state = torch.Tensor(state)
            q_values = self.model(state)
            return torch.argmax(q_values).item()
        
    def train(self, state, action, reward, next_state, done):
        state = torch.Tensor(state)
        next_state = torch.Tensor(next_state)
        target = reward + self.gamma * torch.max(self.model(next_state)) * (1 - done)
        q_values = self.model(state)
        q_values[action] = target
        
        self.optimizer.zero_grad()
        loss = self.criterion(q_values, q_values.detach())
        loss.backward()
        self.optimizer.step()
        
        if self.epsilon > self.epsilon_min:
            self.epsilon *= self.epsilon_decay

# Define the training function
def train_agent(agent, env, episodes):
    for episode in range(episodes):
        env.reset()
        state = env.get_state()
        done = False
        total_reward = 0
        
        while not done:
            action = agent.get_action(state)
            next_state, reward, done = env.take_action(action)
            agent.train(state, action, reward, next_state, done)
            
            state = next_state
            total_reward += reward

            if total_reward > env.max_reward:
                env.max_reward = total_reward
            
        # print(f"Episode: {episode+1}, Total Reward: {total_reward}")

max_reward = 0

def greedy(items, capacity):
    total_value = 0
    for value, weight in items:
        if weight <= capacity:
            total_value += value
            capacity -= weight
    return total_value

if __name__ == '__main__':
    # weights = [1, 2, 3, 4, 5, 1]
    # values = [5, 4, 3, 2, 1, 5]
    # capacity = 10
    n_size = 10000
    if n_size >= 1000:
        episodes = 5
    else:
        episodes = n_size
    weights = []
    values = []
    for _ in range(n_size):
        weights.append(np.random.randint(1, 50))
        values.append(np.random.randint(1, 50))
    capacity = sum(weights) // 4

    items = list(zip(values, weights))
    # sort list by value/weight ratio
    items.sort(key=lambda x: x[0]/x[1], reverse=True)
    env = KnapsackEnvironment(items, capacity)

    dqn_start = time.time()
    agent = DQNAgent(2, 2)

    # begin training
    train_agent(agent, env, episodes)

    # state = env.get_state()
    # done = False

    print(f"DQN Time cost: {time.time() - dqn_start}")
    print(f"Max Reward: {env.max_reward}")

    # total_reward = 0

    # while not done or max_reward < max(values):
    #     action = agent.get_action(state)
    #     next_state, reward, done = env.take_action(action)
    #     max_reward = max(env.max_reward, reward)
    #     if not done or reward < max(values):
    #         state = next_state
        # total_reward += reward


    greedy_res = greedy(items, capacity)
    print(f"Greedy Max Reward: {greedy_res}")